mod api;
mod config;
mod models;
mod services;
mod metrics;

use actix_web::{web, App, HttpServer};
use crate::api::handlers::start_load_test;
use crate::metrics::TestMetrics;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let metrics = Arc::new(TestMetrics::new());
    let should_stop = Arc::new(AtomicU64::new(0));

    println!("Starting Ollama load test server...");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(metrics.clone()))
            .app_data(web::Data::new(should_stop.clone()))
            .service(web::resource("/start-test/{concurrent_users}")
                .route(web::post().to(start_load_test)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
