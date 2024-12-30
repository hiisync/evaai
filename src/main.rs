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

    // Створення безкінечного циклу для періодичної відправки повідомлень
    loop {
        let delay = rand::thread_rng().gen_range(1..=1); // Випадковий інтервал між 30 хвилинами (1800 секунд) і 2 годинами (7200 секунд)
        println!(
            "Waiting for {} minutes before sending the next message",
            delay
        );

        sleep(Duration::from_secs(delay * 60)).await; // Очікуємо потрібний час

        send_message(&bot).await;
    }
}
