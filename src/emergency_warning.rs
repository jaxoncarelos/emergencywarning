use std::sync::mpsc::{Receiver, Sender};

use reqwest;
use serde_derive::Deserialize;
#[derive(Deserialize, Debug)]
pub struct APIResponse {
    pub features: Vec<Properties>,
}

#[derive(Deserialize, Debug)]
pub struct Properties {
    pub properties: FeatureBreakDown,
}

#[derive(Deserialize, Debug)]
pub struct FeatureBreakDown {
    pub event: String,
    pub severity: String,
    pub certainty: String,
    pub areaDesc: String,
}

impl Default for APIResponse {
    fn default() -> APIResponse {
        APIResponse {
            features: vec![Properties {
                properties: FeatureBreakDown {
                    event: "N/A".to_string(),
                    severity: "N/A".to_string(),
                    certainty: "N/A".to_string(),
                    areaDesc: "N/A".to_string(),
                },
            }],
        }
    }
}
pub async fn poll_warning(state: &crate::helper::State, severity: String, tx: Sender<Properties>) {
    // every 5 minutes, check for new warnings
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(300));
    loop {
        interval.tick().await;
        let response = check_warning(state, state.abbreviate()).await.unwrap();
        for feature in response.features {
            if feature.properties.severity == severity {
                tx.send(feature);
            }
        }
    }
}
pub async fn check_warning(_state: &crate::helper::State, abbrev: String) -> Option<APIResponse> {
    let client = reqwest::Client::builder()
        .user_agent("jaxoncarelos@gmail.com")
        .build()
        .unwrap();
    let api_url = format!("https://api.weather.gov/alerts/active?area={}", abbrev);
    let response: APIResponse = match client.get(api_url).send().await {
        Ok(data) => match serde_json::from_str(&data.text().await.unwrap().trim()) {
            Err(e) => {
                println!("Error: {}", e);
                APIResponse::default()
            }
            Ok(json) => json,
        },
        _ => APIResponse::default(),
    };
    Some(response)
}
