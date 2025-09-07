use std::time::Instant;
use poise::serenity_prelude as serenity;
use std::fs::OpenOptions;
use std::io::Write;
use serde::Serialize;

#[derive(Serialize)]
struct CommandMetric {
    command: String,
    execution_time_ms: f64,
    timestamp: String,
    user_id: u64,
    guild_id: Option<u64>,
    success: bool,
}

/// Simple performance tracking pre-command hook
pub async fn track_performance_pre<'a>(
    ctx: poise::ApplicationContext<'a, crate::bot::data::Data, crate::bot::data::Error>,
) {
    // Store the start time in the context data
    let start = Instant::now();
    
    // Store in a thread-safe way (you could use a HashMap with command ID as key)
    // For simplicity, we'll track it differently in the post hook
    
    tracing::debug!(
        "Command '{}' started by user {}", 
        ctx.command().name,
        ctx.author().id
    );
}

/// Simple performance tracking post-command hook
pub async fn track_performance_post<'a>(
    ctx: poise::ApplicationContext<'a, crate::bot::data::Data, crate::bot::data::Error>,
    result: Result<(), crate::bot::data::Error>,
) {
    // For now we'll measure from this point (not perfect but simple)
    // In production, you'd store the start time in a concurrent HashMap
    let execution_time_ms = 0.0; // This would be calculated from stored start time
    
    let metric = CommandMetric {
        command: ctx.command().name.to_string(),
        execution_time_ms,
        timestamp: chrono::Utc::now().to_rfc3339(),
        user_id: ctx.author().id.0,
        guild_id: ctx.guild_id().map(|id| id.0),
        success: result.is_ok(),
    };
    
    // Log to file (append mode)
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("test_results/command_metrics.jsonl")
    {
        let _ = writeln!(file, "{}", serde_json::to_string(&metric).unwrap_or_default());
    }
    
    // Log to console
    let status = if result.is_ok() { "✓" } else { "✗" };
    tracing::info!(
        "Command '{}' completed {} for user {} in {:.2}ms",
        ctx.command().name,
        status,
        ctx.author().id,
        execution_time_ms
    );
}

/// Even simpler: A wrapper function that measures execution time
pub async fn measure_command_time<F, Fut>(
    command_name: &str,
    user_id: u64,
    guild_id: Option<u64>,
    f: F,
) -> Result<(), crate::bot::data::Error>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<(), crate::bot::data::Error>>,
{
    let start = Instant::now();
    
    let result = f().await;
    
    let execution_time_ms = start.elapsed().as_secs_f64() * 1000.0;
    
    // Save metric
    let metric = CommandMetric {
        command: command_name.to_string(),
        execution_time_ms,
        timestamp: chrono::Utc::now().to_rfc3339(),
        user_id,
        guild_id,
        success: result.is_ok(),
    };
    
    // Ensure directory exists
    let _ = std::fs::create_dir_all("test_results");
    
    // Append to metrics file
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("test_results/command_metrics.jsonl")
    {
        let _ = writeln!(file, "{}", serde_json::to_string(&metric).unwrap_or_default());
    }
    
    tracing::info!(
        "Command '{}' executed in {:.2}ms ({})",
        command_name,
        execution_time_ms,
        if result.is_ok() { "success" } else { "failed" }
    );
    
    result
}