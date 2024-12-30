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
    // Method for creating a request object
    pub async fn new(text: &str) -> Self {
        ContentRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: text.to_string(),
                }],
            }],
        }
    }

    // Method for sending the request
    pub async fn send_request(&self, api_key: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
            api_key
        );

        let client = Client::new();
        let response = client
            .post(&url)
            .json(self) // Serialize the request object
            .send()
            .await?;

        // Deserialize the JSON response
        let response_json: ApiResponse = response.json().await?;

        // Extract text from the first candidate and the first part
        if let Some(candidate) = response_json.candidates.get(0) {
            if let Some(part) = candidate.content.parts.get(0) {
                return Ok(part.text.clone());
            }
        }

        // If no response, return an error
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No response found",
        )))
    }
}
