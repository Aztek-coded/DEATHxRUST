# Data Management Guidelines

### Shared State

```rust
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct Data {
    settings: Settings,
    cache: Arc<RwLock<HashMap<String, String>>>,
    db_pool: sqlx::PgPool,
}

impl Data {
    pub async fn get_cached(&self, key: &str) -> Option<String> {
        self.cache.read().await.get(key).cloned()
    }
    
    pub async fn set_cached(&self, key: String, value: String) {
        self.cache.write().await.insert(key, value);
    }
}
```

### Database Integration

```rust
// Using SQLx
pub async fn get_user_data(
    pool: &sqlx::PgPool,
    user_id: i64,
) -> Result<UserData, Error> {
    sqlx::query_as!(
        UserData,
        "SELECT * FROM users WHERE id = $1",
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(Into::into)
}
```

### Caching Strategy

```rust
use moka::future::Cache;

pub struct Data {
    user_cache: Cache<u64, User>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            user_cache: Cache::builder()
                .max_capacity(1000)
                .time_to_live(Duration::from_secs(300))
                .build(),
        }
    }
}
```

## 