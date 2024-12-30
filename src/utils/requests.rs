use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ContentRequest {
    contents: Vec<Content>,
}

#[derive(Serialize)]
pub struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize, Deserialize)]
pub struct Part {
    text: String,
}

#[derive(Deserialize)]
struct ApiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: CandidateContent,
}

#[derive(Deserialize)]
struct CandidateContent {
    parts: Vec<Part>,
}

impl ContentRequest {
    // Метод для створення об'єкта запиту
    pub async fn new(text: &str) -> Self {
        ContentRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: text.to_string(),
                }],
            }],
        }
    }

    // Метод для надсилання запиту
    pub async fn send_request(&self, api_key: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
            api_key
        );

        let client = Client::new();
        let response = client
            .post(&url)
            .json(self) // Серіалізація об'єкта запиту
            .send()
            .await?;

        // Десеріалізація JSON-відповіді
        let response_json: ApiResponse = response.json().await?;

        // Отримуємо текст з першого кандидата та першої частини
        if let Some(candidate) = response_json.candidates.get(0) {
            if let Some(part) = candidate.content.parts.get(0) {
                return Ok(part.text.clone());
            }
        }

        // Якщо відповіді немає, повертаємо помилку
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No response found",
        )))
    }
}
