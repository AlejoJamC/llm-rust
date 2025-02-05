use reqwest::Client;
use crate::models::request::OllamaRequest;
use crate::models::response::OllamaResponse;
use crate::config::Config; // Importar la configuración

pub struct OllamaService {
    client: Client,
    config: Config, // Usar la configuración en lugar de hardcodear valores
}

impl OllamaService {
    pub fn new(config: Config) -> Self {
        Self {
            client: Client::new(),
            config, // Inyectar la configuración
        }
    }

    pub async fn send_prompt(&self, request: OllamaRequest) -> Result<OllamaResponse, reqwest::Error> {
        // Usar la URL de Ollama desde la configuración
        let response = self.client
            .post(&self.config.ollama_url)
            .json(&request)
            .send()
            .await?;

        response.json::<OllamaResponse>().await
    }
}
