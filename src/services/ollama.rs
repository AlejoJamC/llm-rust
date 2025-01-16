use reqwest::Client;
use crate::models::request::OllamaRequest;
use crate::config::Config;

pub struct OllamaService {
    client: Client,
    config: Config,
}

impl OllamaService {
    pub fn new(config: Config) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    pub async fn send_request(&self, payload: OllamaRequest) -> Result<(), reqwest::Error> {
        self.client
            .post(&self.config.ollama_url)
            .json(&payload)
            .send()
            .await?;
        Ok(())
    }
}
