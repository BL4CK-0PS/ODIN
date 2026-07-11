use odin_kernel::KernelError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct RedisClient {
    url: String,
    client: Option<redis::aio::MultiplexedConnection>,
    memory: Arc<RwLock<std::collections::HashMap<String, CacheEntry>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheEntry {
    value: String,
    expires_at: Option<u64>,
}

impl RedisClient {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            client: None,
            memory: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    pub async fn connect(&mut self) -> Result<(), KernelError> {
        let client = redis::Client::open(self.url.as_str())
            .map_err(|e| KernelError::Internal(format!("Redis client creation failed: {}", e)))?;
        match client.get_multiplexed_async_connection().await {
            Ok(conn) => {
                self.client = Some(conn);
                tracing::info!("Redis connected: {}", self.url);
                Ok(())
            }
            Err(e) => {
                tracing::warn!("Redis not available, using in-memory cache: {}", e);
                Ok(())
            }
        }
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>, KernelError> {
        if let Some(ref conn) = self.client {
            let mut conn = conn.clone();
            let result: Option<String> = redis::cmd("GET")
                .arg(key)
                .query_async::<Option<String>>(&mut conn)
                .await
                .map_err(|e| KernelError::Internal(format!("Redis GET failed: {}", e)))?;
            return Ok(result);
        }

        let cache = self.memory.read().await;
        match cache.get(key) {
            Some(entry) => {
                if let Some(expires) = entry.expires_at {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();
                    if now > expires {
                        return Ok(None);
                    }
                }
                Ok(Some(entry.value.clone()))
            }
            None => Ok(None),
        }
    }

    pub async fn set(&self, key: &str, value: &str, ttl_secs: Option<u64>) -> Result<(), KernelError> {
        if let Some(ref conn) = self.client {
            let mut conn = conn.clone();
            let mut cmd = redis::cmd("SET");
            cmd.arg(key).arg(value);
            if let Some(ttl) = ttl_secs {
                cmd.arg("EX").arg(ttl);
            }
            cmd.query_async::<()>(&mut conn)
                .await
                .map_err(|e| KernelError::Internal(format!("Redis SET failed: {}", e)))?;
            return Ok(());
        }

        let expires_at = ttl_secs.map(|ttl| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() + ttl
        });

        let mut cache = self.memory.write().await;
        cache.insert(key.to_string(), CacheEntry {
            value: value.to_string(),
            expires_at,
        });
        Ok(())
    }

    pub async fn del(&self, key: &str) -> Result<(), KernelError> {
        if let Some(ref conn) = self.client {
            let mut conn = conn.clone();
            redis::cmd("DEL")
                .arg(key)
                .query_async::<()>(&mut conn)
                .await
                .map_err(|e| KernelError::Internal(format!("Redis DEL failed: {}", e)))?;
            return Ok(());
        }

        let mut cache = self.memory.write().await;
        cache.remove(key);
        Ok(())
    }

    pub async fn incr(&self, key: &str) -> Result<i64, KernelError> {
        if let Some(ref conn) = self.client {
            let mut conn = conn.clone();
            let result: i64 = redis::cmd("INCR")
                .arg(key)
                .query_async::<i64>(&mut conn)
                .await
                .map_err(|e| KernelError::Internal(format!("Redis INCR failed: {}", e)))?;
            return Ok(result);
        }

        let mut cache = self.memory.write().await;
        let entry = cache.entry(key.to_string()).or_insert_with(|| CacheEntry {
            value: "0".to_string(),
            expires_at: None,
        });
        let val: i64 = entry.value.parse().unwrap_or(0) + 1;
        entry.value = val.to_string();
        Ok(val)
    }

    pub async fn health_check(&self) -> bool {
        if let Some(ref conn) = self.client {
            let mut conn = conn.clone();
            redis::cmd("PING")
                .query_async::<String>(&mut conn)
                .await
                .is_ok()
        } else {
            false
        }
    }
}
