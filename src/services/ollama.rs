use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::models::request::OllamaRequest;
use crate::models::response::OllamaResponse;

pub struct OllamaService {
    client: Client,
    ollama_url: String,
}

impl OllamaService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            ollama_url: "http://localhost:11434/api/generate".to_string(),
        }
    }

    pub async fn send_prompt(&self, request: OllamaRequest) -> Result<OllamaResponse, reqwest::Error> {
        let response = self.client
            .post(&self.ollama_url)
            .json(&request)
            .send()
            .await?;

        response.json::<OllamaResponse>().await
    }
}