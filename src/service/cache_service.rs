use std::time::Duration;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::config::domain::CacheConfig;
use crate::error::{AppError, AppResult};
use crate::service::redis_service::RedisService;

#[async_trait]
pub trait ICacheService: Sync + Send {
    async fn set_string(&self, k: &str, v: &str) -> AppResult<String>;

    async fn get_string(&self, k: &str) -> AppResult<String>;

    async fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> AppResult<String>;

    async fn ttl(&self, k: &str) -> AppResult<i64>;
}

pub struct CacheService {
    pub inner: Box<dyn ICacheService>,
}

impl CacheService {
    pub fn new(cfg: &CacheConfig) -> AppResult<Self> {
        log::info!("cache_type: {}",cfg.cache_type());

        match cfg.cache_type().as_str() {
            // "mem" => {
            //     println!("[abs_admin] cache_type: mem");
            //     Ok(Self {
            //         inner: Box::new(MemService::default()),
            //     })
            // }
            "redis" => {
                if cfg.redis_url().is_none() {
                    log::error!("redis cache must config redis_url");
                    panic!("")
                }
                Ok(Self {
                    inner: Box::new(RedisService::new(&cfg.redis_url().clone().unwrap())),
                })
            }
            e => {
                panic!(
                    "[abs_admin] unknown of cache_type: \"{}\",current support 'mem' or 'redis'",
                    e
                );
            }
        }
    }

    pub async fn set_string(&self, k: &str, v: &str) -> AppResult<String> {
        self.inner.set_string(k, v).await
    }

    pub async fn get_string(&self, k: &str) -> AppResult<String> {
        self.inner.get_string(k).await
    }

    pub async fn set_json<T>(&self, k: &str, v: &T) -> AppResult<String>
        where
            T: Serialize + Sync,
    {
        let data = serde_json::to_string(v);
        if data.is_err() {
            return Err(AppError::from(format!(
                "CacheService set_json fail:{}",
                data.err().unwrap()
            )));
        }
        let data = self.set_string(k, data.unwrap().as_str()).await?;
        Ok(data)
    }

    pub async fn get_json<T>(&self, k: &str) -> AppResult<T>
        where
            T: DeserializeOwned + Sync,
    {
        let mut r = self.get_string(k).await?;
        if r.is_empty() {
            r = "null".to_string();
        }
        let data: serde_json::Result<T> = serde_json::from_str(r.as_str());
        if data.is_err() {
            return Err(AppError::from(format!(
                "CacheService GET fail:{}",
                data.err().unwrap()
            )));
        }
        Ok(data.unwrap())
    }

    pub async fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> AppResult<String> {
        self.inner.set_string_ex(k, v, ex).await
    }

    pub async fn ttl(&self, k: &str) -> AppResult<i64> {
        self.inner.ttl(k).await
    }
}
