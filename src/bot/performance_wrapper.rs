use std::time::Instant;
use std::fs::OpenOptions;
use std::io::Write;
use serde::Serialize;

#[derive(Serialize)]
struct Metric {
    command: String,
    response_time_ms: f64,
    timestamp: String,
    user_id: u64,
    guild_id: Option<u64>,
    success: bool,
}

/// Wrap ALL commands with automatic performance tracking
/// This is the SIMPLEST way - just wrap the command execution
pub async fn wrap_command_with_metrics<'a>(
    ctx: poise::ApplicationContext<'a, crate::bot::data::Data, crate::bot::data::Error>,
) -> Result<(), crate::bot::data::Error> {
    let start = Instant::now();
    let command_name = ctx.command().name.to_string();
    let user_id = ctx.author().id.0;
    let guild_id = ctx.guild_id().map(|id| id.0);
    
    // Execute the actual command
    let result = (ctx.command().action.slash.unwrap())(ctx).await;
    
    // Calculate response time
    let response_time_ms = start.elapsed().as_secs_f64() * 1000.0;
    
    // Log metric
    let metric = Metric {
        command: command_name.clone(),
        response_time_ms,
        timestamp: chrono::Utc::now().to_rfc3339(),
        user_id,
        guild_id,
        success: result.is_ok(),
    };
    
    // Save to file (create dir if needed)
    let _ = std::fs::create_dir_all("test_results");
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("test_results/command_metrics.jsonl")
    {
        let _ = writeln!(file, "{}", serde_json::to_string(&metric).unwrap_or_default());
    }
    
    // Log to console
    tracing::info!(
        "Command '{}' executed in {:.2}ms ({})",
        command_name,
        response_time_ms,
        if result.is_ok() { "✓" } else { "✗" }
    );
    
    result
}