use actix_web::{web, HttpResponse};
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use crate::metrics::TestMetrics;
use crate::services::load_test::LoadTestService;
use crate::config::Config;

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

// Estructura para recibir el prompt en el cuerpo de la solicitud
#[derive(Deserialize)]
pub struct PromptRequest {
    prompt: String,
}

// Estructura para devolver la respuesta de Ollama
#[derive(Serialize)]
pub struct OllamaResponse {
    response: String,
}

// Nueva función para consultar Ollama
pub async fn query_ollama(prompt: web::Json<PromptRequest>) -> HttpResponse {
    let client = Client::new();
    let ollama_url = "http://localhost:11434/api/generate"; // URL de la API de Ollama

    // Enviar la solicitud a Ollama
    let response = client.post(ollama_url)
        .json(&serde_json::json!({
            "prompt": prompt.prompt,
            "model": "llama2" // Ajusta el modelo según sea necesario
        }))
        .send()
        .await;

    // Manejar la respuesta de Ollama
    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                // Parsear la respuesta JSON de Ollama
                let ollama_response: OllamaResponse = resp.json().await.unwrap_or_else(|_| OllamaResponse { 
                    response: "Error parsing response".to_string() 
                });
                HttpResponse::Ok().json(ollama_response)
            } else {
                HttpResponse::InternalServerError().body("Failed to query Ollama")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to send request to Ollama"),
    }
}