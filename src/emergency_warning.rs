use reqwest;
use serde_derive::Deserialize;
#[derive(Deserialize)]
pub struct APIResponse {
    pub features: Vec<Properties>,
}

#[derive(Deserialize)]
pub struct Properties {
    pub properties: FeatureBreakDown,
}

#[derive(Deserialize)]
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
