mod handlers;
mod utils;

use handlers::gemini::send_message;
use rand::Rng;
use teloxide::prelude::*;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting the bot...");
    let token = std::env::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN not set");

    let bot = Bot::new(token);

    // Create an infinite loop for periodic message sending
    loop {
        let delay = rand::thread_rng().gen_range(1..=1); // Random interval between 30 minutes (1800 seconds) and 2 hours (7200 seconds)
        println!(
            "Waiting for {} minutes before sending the next message",
            delay
        );

        sleep(Duration::from_secs(delay * 60)).await; // Wait for the required time

        send_message(&bot).await;
    }
}
