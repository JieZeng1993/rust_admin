use std::time::Duration;

use async_trait::async_trait;
use redis::aio::Connection;
use redis::RedisResult;

use crate::error::{AppError, AppResult};
use crate::service::cache_service::ICacheService;

///Redis Cache service
pub struct RedisService {
    pub client: redis::Client,
}

impl RedisService {
    pub fn new(url: &str) -> Self {
        log::info!("connect redis ({})...", url);
        let client = redis::Client::open(url).unwrap();
        log::info!("connect redis success!");
        Self { client }
    }

    pub async fn get_conn(&self) -> AppResult<Connection> {
        let conn = self.client.get_async_connection().await;
        if conn.is_err() {
            let err = format!("RedisService connect fail:{}", conn.err().unwrap());
            log::error!("{}", err);
            return Err(AppError::from(err));
        }
        return Ok(conn.unwrap());
    }
}

#[async_trait]
impl ICacheService for RedisService {
    async fn set_string(&self, k: &str, v: &str) -> AppResult<String> {
        let k = k.to_string();
        let v = v.to_string();
        return self.set_string_ex(&k, &v, None).await;
    }

    async fn get_string(&self, k: &str) -> AppResult<String> {
        let k = k.to_string();
        let mut conn = self.get_conn().await?;
        let result: RedisResult<Option<String>> =
            redis::cmd("GET").arg(&[&k]).query_async(&mut conn).await;
        return match result {
            Ok(v) => Ok(v.unwrap_or_default()),
            Err(e) => Err(AppError::from(format!(
                "RedisService get_string({}) fail:{}",
                k,
                e.to_string()
            ))),
        };
    }

    ///set_string Automatically expire
    async fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> AppResult<String> {
        let k = k.to_string();
        let v = v.to_string();
        let mut conn = self.get_conn().await?;
        return if ex.is_none() {
            match redis::cmd("SET").arg(&[k, v]).query_async(&mut conn).await {
                Ok(v) => Ok(v),
                Err(e) => Err(AppError::from(format!(
                    "RedisService set_string_ex fail:{}",
                    e.to_string()
                ))),
            }
        } else {
            match redis::cmd("SET")
                .arg(&[&k, &v, "EX", &ex.unwrap().as_secs().to_string()])
                .query_async(&mut conn)
                .await
            {
                Ok(v) => Ok(v),
                Err(e) => Err(AppError::from(format!(
                    "RedisService set_string_ex fail:{}",
                    e.to_string()
                ))),
            }
        };
    }

    ///set_string Automatically expire
    async fn ttl(&self, k: &str) -> AppResult<i64> {
        let k = k.to_string();
        let mut conn = self.get_conn().await?;
        return match redis::cmd("TTL").arg(&[k]).query_async(&mut conn).await {
            Ok(v) => Ok(v),
            Err(e) => Err(AppError::from(format!(
                "RedisService ttl fail:{}",
                e.to_string()
            ))),
        };
    }
}
