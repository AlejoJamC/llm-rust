use std::sync::atomic::{AtomicU64, Ordering};

pub struct TestMetrics {
    pub total_latency: AtomicU64,
    pub total_requests: AtomicU64,
    pub max_latency: AtomicU64,
    pub failed_requests: AtomicU64,
}

impl TestMetrics {
    pub fn new() -> Self {
        TestMetrics {
            total_latency: AtomicU64::new(0),
            total_requests: AtomicU64::new(0),
            max_latency: AtomicU64::new(0),
            failed_requests: AtomicU64::new(0),
        }
    }

    pub fn update_metrics(&self, latency: u64) {
        self.total_latency.fetch_add(latency, Ordering::Relaxed);
        self.max_latency.fetch_max(latency, Ordering::Relaxed);
        self.total_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_failed(&self) {
        self.failed_requests.fetch_add(1, Ordering::Relaxed);
    }
}
