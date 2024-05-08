use crate::{emergency_warning::APIResponse, helper::State};

mod emergency_warning;
mod helper;
#[tokio::main]
async fn main() {
    println!("Tornado warning initiated");
    println!("Enter state you would like to check warnings for or multiple seperated by commas: ");
    let mut states = String::new();
    std::io::stdin()
        .read_line(&mut states)
        .expect("Invalid Input");
    let mut state_enums: Vec<State> = Vec::new();
    for state in states.split(',') {
        println!("state: {}", state);
        state_enums.push(helper::state_to_enum(state.trim()));
    }
    let mut responses: Vec<APIResponse> = Vec::new();
    for state in state_enums {
        responses.push(
            emergency_warning::check_warning(&state, state.abbreviate())
                .await
                .unwrap(),
        );
    }
    for response in responses {
        for feature in response.features {
            println!(
                "Event: {}\nSeverity: {}\nCertainty: {}\nArea Description: {}\n",
                feature.properties.event,
                feature.properties.severity,
                feature.properties.certainty,
                feature.properties.areaDesc
            );
        }
    }
}
