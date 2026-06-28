use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenCache {
    pub user_id: i64,
    pub username: String,
    pub exp: i64,
}

pub async fn set_token(
    redis: &ConnectionManager,
    token: &str,
    user_id: i64,
    username: &str,
    exp: i64,
    ttl: i64,
) -> Result<(), redis::RedisError> {
    // redis链接
    let mut conn = redis.clone();

    // 存入redis的缓存数据
    let cache = TokenCache {
        user_id,
        username: username.to_string(),
        exp,
    };

    // 序列化成json字符串
    let cache_json = serde_json::to_string(&cache).map_err(|e| {
        redis::RedisError::from(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("erialization error: {}", e),
        ))
    })?;

    let key = format!("token: {token}");
    conn.set_ex::<_, _, ()>(&key, cache_json, ttl as u64)
        .await?;
    Ok(())
}
/*
获取token
*/
pub async fn get_token(
    redis: &ConnectionManager,
    token: &str,
) -> Result<Option<TokenCache>, redis::RedisError> {
    let mut conn = redis.clone();
    let key = format!("token: {token}");
    let result: Option<String> = conn.get(&key).await?;

    match result {
        Some(cache_json) => {
            // 反序列化为TokenCache结构体
            let cache: TokenCache = serde_json::from_str(&cache_json).map_err(|e| {
                redis::RedisError::from(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("deserialization error: {}", e),
                ))
            })?;
            Ok(Some(cache))
        }
        None => Ok(None),
    }
}

pub async fn delete_token(redis: &ConnectionManager, token: &str) -> Result<(), redis::RedisError> {
    let mut conn = redis.clone();
    let key = format!("token: {token}");
    conn.del::<_, ()>(&key).await?;
    Ok(())
}

pub async fn delete_user_tokens(
    redis: &ConnectionManager,
    user_id: i64,
) -> Result<(), redis::RedisError> {
    let mut conn = redis.clone();
    let pattern = "token:*".to_string();
    let keys: Vec<String> = conn.keys(&pattern).await?;

    for key in keys {
        let result: Option<String> = conn.get(&key).await?;
        if let Some(cache_json) = result {
            if let Ok(cache) = serde_json::from_str::<TokenCache>(&cache_json) {
                if cache.user_id == user_id {
                    conn.del::<_, ()>(&key).await?;
                }
            }
        }
    }

    Ok(())
}
