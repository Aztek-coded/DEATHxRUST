/// Simplest possible performance tracking for Poise
/// Add this to your bot's Data struct and framework hooks

use std::time::Instant;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::fs::OpenOptions;
use std::io::Write;

/// Add this to your Data struct in bot/data.rs
pub struct CommandTimings {
    active: Arc<RwLock<HashMap<String, Instant>>>,
}

impl CommandTimings {
    pub fn new() -> Self {
        Self {
            active: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Call in pre_command hook
    pub async fn start(&self, command_id: String) {
        let mut timings = self.active.write().await;
        timings.insert(command_id, Instant::now());
    }
    
    /// Call in post_command hook
    pub async fn end(&self, command_id: String, command_name: &str, user_id: u64, guild_id: Option<u64>) -> Option<f64> {
        let mut timings = self.active.write().await;
        
        if let Some(start) = timings.remove(&command_id) {
            let ms = start.elapsed().as_secs_f64() * 1000.0;
            
            // Log to file
            let metric = format!(
                r#"{{"command":"{}","ms":{},"user_id":{},"guild_id":{},"timestamp":"{}"}}"#,
                command_name,
                ms,
                user_id,
                guild_id.map(|id| id.to_string()).unwrap_or_else(|| "null".to_string()),
                chrono::Utc::now().to_rfc3339()
            );
            
            let _ = std::fs::create_dir_all("test_results");
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open("test_results/command_metrics.jsonl")
            {
                let _ = writeln!(file, "{}", metric);
            }
            
            tracing::info!("Command '{}' took {:.2}ms", command_name, ms);
            Some(ms)
        } else {
            None
        }
    }
}

/// Example of how to use in framework.rs:
/// 
/// In your Data struct:
/// ```
/// pub struct Data {
///     pub timings: CommandTimings,
///     // ... other fields
/// }
/// ```
/// 
/// In framework setup:
/// ```
/// pre_command: |ctx| {
///     Box::pin(async move {
///         let command_id = format!("{}-{}", ctx.id(), ctx.command().name);
///         ctx.data().timings.start(command_id).await;
///     })
/// },
/// post_command: |ctx| {
///     Box::pin(async move {
///         let command_id = format!("{}-{}", ctx.id(), ctx.command().name);
///         ctx.data().timings.end(
///             command_id,
///             ctx.command().name,
///             ctx.author().id.0,
///             ctx.guild_id().map(|id| id.0)
///         ).await;
///     })
/// },
/// ```