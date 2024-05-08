use crate::{
    emergency_warning::{check_warning, poll_warning, APIResponse, Properties},
    helper::State,
};
use notify_rust::Notification;
use std::sync::mpsc::Receiver;

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
    println!("Would you like to poll events or check once? (poll/once): ");
    let mut poll = String::new();
    std::io::stdin()
        .read_line(&mut poll)
        .expect("Invalid Input");

    let mut state_enums: Vec<State> = Vec::new();
    for state in states.split(',') {
        println!("state: {}", state);
        state_enums.push(helper::state_to_enum(state.trim()));
    }
    let mut rx_vec: Vec<Receiver<Properties>> = Vec::new();
    for state in state_enums {
        match poll.trim() {
            "poll" => {
                let (tx, rx) = std::sync::mpsc::channel();
                rx_vec.push(rx);
                tokio::spawn(async move {
                    poll_warning(&state, "Severe".to_string(), tx).await;
                });
            }
            "once" => {
                println!(
                    "{:?}",
                    emergency_warning::check_warning(&state, state.abbreviate())
                        .await
                        .unwrap(),
                );
            }
            _ => panic!("Invalid poll option"),
        }
    }
    for rx in rx_vec {
        for received in rx {
            println!("Event: {}", received.properties.event);
            println!("Severity: {}", received.properties.severity);
            println!("Certainty: {}", received.properties.certainty);
            println!("Area Description: {}", received.properties.areaDesc);
            Notification::new()
                .summary(&received.properties.event)
                .body(&received.properties.areaDesc)
                .show()
                .unwrap();
        }
    }
}
