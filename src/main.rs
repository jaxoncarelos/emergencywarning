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
                let check_warning = emergency_warning::check_warning(&state, state.abbreviate())
                    .await
                    .unwrap();
                for feature in check_warning.features {
                    helper::pretty_print(feature);
                }
            }
            _ => panic!("Invalid poll option"),
        }
    }
    let mut lastFive: Vec<Properties> = Vec::new();
    for rx in rx_vec {
        for received in rx {
            if lastFive.contains(&received) {
                continue;
            }
            lastFive.push(received.clone());
            if lastFive.len() > 5 {
                lastFive.remove(0);
            }
            helper::pretty_print(received);
        }
    }
}
