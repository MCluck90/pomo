use clap::Clap;
use notify_rust::{Notification, Timeout};
use rand::prelude::*;
use std::time::Duration;
use tokio::time::delay_for;

#[derive(Clap)]
#[clap(version = "1.0", author = "Mike C. <mcluck90@gmail.com>")]
struct Opts {
    #[clap(about = "How many minutes to run the timer for")]
    time: u64,

    #[clap(short, long, about = "Are you resting?")]
    rest: bool,
}

#[cfg(not(target_os = "macos"))]
fn show_notification(
    body_message: &str,
    resources: std::path::PathBuf,
) -> Result<(), notify_rust::error::Error> {
    Notification::new()
        .appname("Pomo")
        .summary("Break time!")
        .body(body_message)
        .image_path(&format!("file:///{}", resources.to_str().unwrap()))
        .timeout(Timeout::Milliseconds(6000))
        .show()
}

#[cfg(target_os = "macos")]
fn show_notification(
    body_message: &str,
    _resources: std::path::PathBuf,
) -> Result<(), notify_rust::error::Error> {
    Notification::new()
        .appname("Pomo")
        .summary("Break time!")
        .body(body_message)
        .timeout(Timeout::Milliseconds(6000))
        .show()
        .map(|_| ())
}

#[tokio::main]
async fn main() {
    let opts = Opts::parse();

    let resources = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("resources/tomato.png");

    if opts.time > 0 {
        println!(
            "See you in {} minute{}!",
            opts.time,
            if opts.time > 1 { "s" } else { "" }
        );
        delay_for(Duration::from_secs(opts.time * 60)).await;
    }

    let body_message = if opts.rest {
        let responses = [
            "Rest is over. Get back to it!",
            "Hope you had a good break.",
            "Get on it, slacker!",
        ];
        let mut rng = rand::thread_rng();
        let index: usize = rng.gen::<usize>() % responses.len();
        responses[index]
    } else {
        let responses = [
            "Time to take a break. Consider stretching.",
            "Whatever you're doing, stop doing it. Take a break.",
            "It's break time, bud",
        ];
        let mut rng = rand::thread_rng();
        let index: usize = rng.gen::<usize>() % responses.len();
        responses[index]
    };

    loop {
        let result = show_notification(body_message, resources.clone());

        match result {
            Ok(_) => break,
            Err(_) => {
                println!("Error, retrying");
                delay_for(Duration::from_millis(200)).await;
            }
        }
    }
}
