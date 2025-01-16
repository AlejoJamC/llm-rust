use actix_web::{web, HttpResponse};
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use crate::metrics::TestMetrics;
use crate::services::load_test::LoadTestService;
use crate::config::Config;

pub async fn start_load_test(
    concurrent_users: web::Path<usize>,
    metrics: web::Data<Arc<TestMetrics>>,
    should_stop: web::Data<Arc<AtomicU64>>,
) -> HttpResponse {
    let config = Config::default();
    let load_test_service = LoadTestService::new(config);
    
    let results = load_test_service.run_test(
        concurrent_users.into_inner(),
        metrics.into_inner(),
        should_stop.into_inner(),
    )
    .await;
    
    HttpResponse::Ok().json(results)
}
