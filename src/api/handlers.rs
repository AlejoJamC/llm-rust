use actix_web::{web, HttpResponse};
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use reqwest::Client;
use crate::metrics::TestMetrics;
use crate::services::load_test::LoadTestService;
use crate::config::Config;
use crate::models::request::OllamaRequest; // Reutilizando la estructura existente
use crate::models::response::TestResults;  // Reutilizando la estructura existente

// Función existente para iniciar la prueba de carga
pub async fn start_load_test(
    concurrent_users: web::Path<usize>,
    metrics: web::Data<Arc<TestMetrics>>,
    should_stop: web::Data<Arc<AtomicU64>>,
) -> HttpResponse {
    let config = Config::default();
    let load_test_service = LoadTestService::new(config);
    
    let results = load_test_service.run_test(
        concurrent_users.into_inner(),
        metrics.get_ref().clone(),  // Usando get_ref() para obtener la referencia Arc 
        should_stop.get_ref().clone(),
    )
    .await;
    
    HttpResponse::Ok().json(results)
}

// Nueva función para consultar Ollama
pub async fn query_ollama(prompt: web::Json<OllamaRequest>) -> HttpResponse {
    let client = Client::new();
    let ollama_url = "http://localhost:11434/api/generate"; // URL de la API de Ollama

    // Enviar la solicitud a Ollama
    let response = client.post(ollama_url)
        .json(&prompt.into_inner()) // Usamos la estructura OllamaRequest directamente
        .send()
        .await;

    // Manejar la respuesta de Ollama
    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                // Parsear la respuesta JSON de Ollama
                let ollama_response: String = resp.text().await.unwrap_or_else(|_| {
                    "Error parsing response".to_string()
                });
                HttpResponse::Ok().json(ollama_response)
            } else {
                HttpResponse::InternalServerError().body("Failed to query Ollama")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to send request to Ollama"),
    }
}