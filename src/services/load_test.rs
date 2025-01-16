use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::time::{Duration, Instant};
use futures::future;
use crate::metrics::TestMetrics;
use crate::models::request::OllamaRequest;
use crate::models::response::TestResults;
use crate::services::ollama::OllamaService;
use crate::config::Config;

pub struct LoadTestService {
    ollama_service: OllamaService,
    config: Config,
}

impl LoadTestService {
    pub fn new(config: Config) -> Self {
        Self {
            ollama_service: OllamaService::new(config.clone()),
            config,
        }
    }

    async fn run_single_user(
        ollama_service: OllamaService,
        metrics: Arc<TestMetrics>,
        should_stop: Arc<AtomicU64>,
        config: Config,
    ) {
        while should_stop.load(Ordering::Relaxed) == 0 {
            let start = Instant::now();
            let request = OllamaRequest {
                model: config.default_model.clone(),
                prompt: "Tell me a short joke".to_string(),
            };

            match ollama_service.send_request(request).await {
                Ok(_) => {
                    let latency = start.elapsed().as_millis() as u64;
                    metrics.update_metrics(latency);

                    if latency > config.max_latency_threshold_ms {
                        should_stop.store(1, Ordering::Relaxed);
                        break;
                    }
                }
                Err(_) => {
                    metrics.increment_failed();
                }
            }

            tokio::time::sleep(Duration::from_millis(config.request_interval_ms)).await;
        }
    }

    pub async fn run_test(
        &self,
        concurrent_users: usize,
        metrics: Arc<TestMetrics>,
        should_stop: Arc<AtomicU64>,
    ) -> TestResults {
        let mut handles = Vec::with_capacity(concurrent_users);

        for _ in 0..concurrent_users {
            let metrics = Arc::clone(&metrics);
            let should_stop = Arc::clone(&should_stop);
            let ollama_service = self.ollama_service.clone();
            let config = self.config.clone();

            handles.push(tokio::spawn(async move {
                Self::run_single_user(
                    ollama_service,
                    metrics,
                    should_stop,
                    config,
                ).await;
            }));
        }

        // Wait for all users to complete
        future::join_all(handles).await;

        self.calculate_results(&metrics)
    }

    fn calculate_results(&self, metrics: &TestMetrics) -> TestResults {
        let total_requests = metrics.total_requests.load(Ordering::Relaxed);
        let total_latency = metrics.total_latency.load(Ordering::Relaxed);
        let avg_latency = if total_requests > 0 {
            total_latency as f64 / total_requests as f64
        } else {
            0.0
        };

        TestResults {
            total_requests,
            avg_latency_ms: avg_latency,
            max_latency_ms: metrics.max_latency.load(Ordering::Relaxed),
            failed_requests: metrics.failed_requests.load(Ordering::Relaxed),
        }
    }
}
