use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct RpcConfig {
    pub url: String,
    pub username: String,
    pub password: String,
    pub timeout_secs: u64,
    pub pool_max_idle_per_host: usize,
    pub pool_idle_timeout_secs: u64,
    pub tcp_keepalive_secs: u64,
}

impl RpcConfig {
    pub fn new(url: String, username: String, password: String) -> Self {
        Self {
            url,
            username,
            password,
            timeout_secs: 30,
            pool_max_idle_per_host: 4,
            pool_idle_timeout_secs: 300,
            tcp_keepalive_secs: 60,
        }
    }

    pub fn with_timeout(mut self, timeout_secs: u64) -> Self {
        self.timeout_secs = timeout_secs;
        self
    }

    pub fn with_pool_max_idle_per_host(mut self, pool_max_idle_per_host: usize) -> Self {
        self.pool_max_idle_per_host = pool_max_idle_per_host.max(1);
        self
    }

    pub fn with_pool_idle_timeout(mut self, pool_idle_timeout_secs: u64) -> Self {
        self.pool_idle_timeout_secs = pool_idle_timeout_secs;
        self
    }

    pub fn with_tcp_keepalive(mut self, tcp_keepalive_secs: u64) -> Self {
        self.tcp_keepalive_secs = tcp_keepalive_secs;
        self
    }

    pub fn with_connection_pool(
        mut self,
        pool_max_idle_per_host: usize,
        pool_idle_timeout_secs: u64,
        tcp_keepalive_secs: u64,
    ) -> Self {
        self.pool_max_idle_per_host = pool_max_idle_per_host.max(1);
        self.pool_idle_timeout_secs = pool_idle_timeout_secs;
        self.tcp_keepalive_secs = tcp_keepalive_secs;
        self
    }

    pub fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout_secs)
    }

    pub fn pool_idle_timeout(&self) -> Duration {
        Duration::from_secs(self.pool_idle_timeout_secs)
    }

    pub fn tcp_keepalive(&self) -> Duration {
        Duration::from_secs(self.tcp_keepalive_secs)
    }
}

#[derive(Debug, Serialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: serde_json::Value,
}

impl JsonRpcRequest {
    pub fn new(method: String, params: serde_json::Value) -> Self {
        Self {
            jsonrpc: "1.0".to_string(),
            id: "blockchain-client".to_string(),
            method,
            params,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct JsonRpcResponse<T> {
    pub result: Option<T>,
    pub error: Option<JsonRpcError>,
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::RpcConfig;
    use std::time::Duration;

    #[test]
    fn rpc_config_uses_pooling_defaults() {
        let config = RpcConfig::new(
            "http://localhost:9332".to_string(),
            "user".to_string(),
            "pass".to_string(),
        );

        assert_eq!(config.timeout(), Duration::from_secs(30));
        assert_eq!(config.pool_max_idle_per_host, 4);
        assert_eq!(config.pool_idle_timeout(), Duration::from_secs(300));
        assert_eq!(config.tcp_keepalive(), Duration::from_secs(60));
    }

    #[test]
    fn rpc_config_allows_pooling_overrides() {
        let config = RpcConfig::new(
            "http://localhost:9332".to_string(),
            "user".to_string(),
            "pass".to_string(),
        )
        .with_connection_pool(8, 600, 120)
        .with_timeout(45);

        assert_eq!(config.timeout(), Duration::from_secs(45));
        assert_eq!(config.pool_max_idle_per_host, 8);
        assert_eq!(config.pool_idle_timeout(), Duration::from_secs(600));
        assert_eq!(config.tcp_keepalive(), Duration::from_secs(120));
    }
}
