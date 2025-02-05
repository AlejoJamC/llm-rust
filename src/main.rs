mod api;
mod config;
mod models;
mod services;
mod metrics;

use actix_web::{web, App, HttpServer};
use crate::api::handlers::{start_load_test, query_ollama};
use crate::metrics::TestMetrics;
use crate::config::Config; // Importar la configuración
use std::sync::Arc;
use std::sync::atomic::AtomicU64;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let metrics = Arc::new(TestMetrics::new());
    let should_stop = Arc::new(AtomicU64::new(0));
    let config = Config::default(); // Crear una instancia de Config

    println!("Starting Ollama load test server...");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(metrics.clone()))
            .app_data(web::Data::new(should_stop.clone()))
            .app_data(web::Data::new(config.clone())) // Pasar la configuración a los handlers
            .service(web::resource("/start-test/{concurrent_users}")
                .route(web::post().to(start_load_test)))
            .service(web::resource("/query-ollama")
                .route(web::post().to(query_ollama)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}