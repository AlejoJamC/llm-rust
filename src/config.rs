#[derive(Clone)]
pub struct Config {
    pub ollama_url: String,
    pub default_model: String,
    pub request_interval_ms: u64,
    pub max_latency_threshold_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ollama_url: "http://localhost:11434/api/generate".to_string(),
            default_model: "llama2".to_string(),
            request_interval_ms: 100,
            max_latency_threshold_ms: 500,
        }
    }
}
