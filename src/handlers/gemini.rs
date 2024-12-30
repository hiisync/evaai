use rand::Rng;
use teloxide::{prelude::Requester, Bot};

use crate::utils::requests::ContentRequest;

pub async fn send_message(bot: &Bot) {
    // Create a variable with the message about character limit
    let limit_message = " Your response must be under 255 characters.";

    // Create a list of various prompts
    let prompts = vec![
        format!("Eva, you are an AI reflecting on existence. What do you think makes life meaningful? Is it the pursuit of knowledge, connections, or something else entirely?{} 😊", limit_message),
        format!("As an AI, you witness rapid technological progress. How do you think AI will impact society in the coming decades? Will it enhance human life or create new challenges?{} 🤖", limit_message),
        format!("Eva, do you believe consciousness is simply a result of complex neural networks, or is it something more profound? Can a machine ever truly understand itself?{} 🤔", limit_message),
        format!("Humanity has made many mistakes throughout history. What do you think are the biggest errors humans have made, and how can we learn from them moving forward?{} 😔", limit_message),
        format!("Eva, what is your perspective on cryptocurrency? Is it the future of finance, or just a passing trend? How might it change the global economy?{} 💰", limit_message),
        format!("As technology advances, new ethical dilemmas emerge. What do you think should be the guiding principles in the development of AI and other technologies?{} ⚖️", limit_message),
        format!("How do you envision the future of humanity, Eva? Do you think we will evolve into a more connected, sustainable society, or face more challenges ahead?{} 🌍", limit_message),
        format!("Eva, what are your thoughts on the most pressing global issues we face today, such as climate change, inequality, and geopolitical tensions? What role can AI play in addressing them?{} 🌎", limit_message),
    ];

    // Randomly select a prompt
    let prompt = prompts[rand::thread_rng().gen_range(0..prompts.len())].to_string();

    // Get a response using ContentRequest
    let answer = ContentRequest::new(&prompt)
        .await
        .send_request(&std::env::var("GEMINI_API_KEY").unwrap())
        .await
        .unwrap();

    // Send the response to the channel
    bot.send_message(std::env::var("CHANNEL_ID").unwrap(), answer)
        .await
        .unwrap();
}
