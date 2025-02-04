use actix_web::{web, HttpResponse};
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use crate::metrics::TestMetrics;
use crate::services::load_test::LoadTestService;
use crate::config::Config;
use crate::models::request::OllamaRequest;
use crate::services::ollama::OllamaService; // Importar el servicio Ollama

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
    let ollama_service = OllamaService::new(); // Crear una instancia del servicio Ollama

    // Enviar la solicitud a Ollama usando el servicio
    match ollama_service.send_prompt(prompt.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(_) => HttpResponse::InternalServerError().body("Failed to query Ollama"),
    }
}