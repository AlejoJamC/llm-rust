use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TestResults {
    pub total_requests: u64,
    pub avg_latency_ms: f64,
    pub max_latency_ms: u64,
    pub failed_requests: u64,
}

#[derive(Debug, Serialize)]
pub struct OllamaResponse {
    pub response: String,
    pub model: String,
    pub created_at: String,
}
