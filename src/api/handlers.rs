use actix_web::{web, HttpResponse};
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use crate::metrics::TestMetrics;
use crate::services::load_test::LoadTestService;
use crate::config::Config; // Importar la configuración
use crate::models::request::OllamaRequest;
use crate::services::ollama::OllamaService;

// Función existente para iniciar la prueba de carga
pub async fn start_load_test(
    concurrent_users: web::Path<usize>,
    metrics: web::Data<Arc<TestMetrics>>,
    should_stop: web::Data<Arc<AtomicU64>>,
    config: web::Data<Config>, // Recibir la configuración
) -> HttpResponse {
    let load_test_service = LoadTestService::new(config.into_inner());
    
    let results = load_test_service.run_test(
        concurrent_users.into_inner(),
        metrics.get_ref().clone(),
        should_stop.get_ref().clone(),
    )
    .await;
    
    HttpResponse::Ok().json(results)
}

// Nueva función para consultar Ollama
pub async fn query_ollama(
    prompt: web::Json<OllamaRequest>,
    config: web::Data<Config>, // Recibir la configuración
) -> HttpResponse {
    let ollama_service = OllamaService::new(config.into_inner()); // Pasar la configuración al servicio

    // Enviar la solicitud a Ollama usando el servicio
    match ollama_service.send_prompt(prompt.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(_) => HttpResponse::InternalServerError().body("Failed to query Ollama"),
    }
}
