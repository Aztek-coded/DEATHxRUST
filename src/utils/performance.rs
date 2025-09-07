use std::time::Instant;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use std::path::Path;
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandMetrics {
    pub command: String,
    pub subcommand: Option<String>,
    pub execution_time_ms: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub user_id: u64,
    pub guild_id: Option<u64>,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Clone)]
pub struct PerformanceTracker {
    metrics: Arc<RwLock<Vec<CommandMetrics>>>,
    active_timers: Arc<RwLock<HashMap<String, Instant>>>,
}

impl PerformanceTracker {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(Vec::new())),
            active_timers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Start timing a command execution
    pub async fn start_timer(&self, command_id: String) {
        let mut timers = self.active_timers.write().await;
        timers.insert(command_id, Instant::now());
    }

    /// End timing and record metrics
    pub async fn end_timer(
        &self,
        command_id: String,
        command: String,
        subcommand: Option<String>,
        user_id: u64,
        guild_id: Option<u64>,
        success: bool,
        error: Option<String>,
    ) -> Option<f64> {
        let mut timers = self.active_timers.write().await;
        
        if let Some(start_time) = timers.remove(&command_id) {
            let execution_time_ms = start_time.elapsed().as_secs_f64() * 1000.0;
            
            let metric = CommandMetrics {
                command,
                subcommand,
                execution_time_ms,
                timestamp: chrono::Utc::now(),
                user_id,
                guild_id,
                success,
                error,
            };
            
            let mut metrics = self.metrics.write().await;
            metrics.push(metric.clone());
            
            // Keep only last 1000 metrics in memory
            if metrics.len() > 1000 {
                let drain_end = metrics.len() - 1000;
                metrics.drain(0..drain_end);
            }
            
            // Async save to file
            let _ = self.save_metrics_to_file(metric).await;
            
            Some(execution_time_ms)
        } else {
            None
        }
    }

    /// Save metrics to file for the Python scripts to read
    async fn save_metrics_to_file(&self, metric: CommandMetrics) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new("test_results/rust_performance_metrics.jsonl");
        
        // Ensure directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        // Append metric as JSON line
        let json_line = serde_json::to_string(&metric)? + "\n";
        
        // Append to file
        let mut contents = if path.exists() {
            fs::read_to_string(path).await.unwrap_or_default()
        } else {
            String::new()
        };
        
        contents.push_str(&json_line);
        
        // Keep only last 10000 lines
        let lines: Vec<&str> = contents.lines().collect();
        let keep_lines = if lines.len() > 10000 {
            &lines[lines.len() - 10000..]
        } else {
            &lines[..]
        };
        
        let final_content = keep_lines.join("\n") + "\n";
        fs::write(path, final_content).await?;
        
        Ok(())
    }

    /// Get average response time for a command
    pub async fn get_average_response_time(&self, command: &str, subcommand: Option<&str>) -> Option<f64> {
        let metrics = self.metrics.read().await;
        
        let relevant_metrics: Vec<&CommandMetrics> = metrics
            .iter()
            .filter(|m| m.command == command && m.subcommand.as_deref() == subcommand)
            .collect();
        
        if relevant_metrics.is_empty() {
            return None;
        }
        
        let sum: f64 = relevant_metrics.iter().map(|m| m.execution_time_ms).sum();
        Some(sum / relevant_metrics.len() as f64)
    }

    /// Get metrics summary
    pub async fn get_summary(&self) -> HashMap<String, serde_json::Value> {
        let metrics = self.metrics.read().await;
        
        let total = metrics.len();
        let successful = metrics.iter().filter(|m| m.success).count();
        let failed = total - successful;
        
        let avg_time = if total > 0 {
            metrics.iter().map(|m| m.execution_time_ms).sum::<f64>() / total as f64
        } else {
            0.0
        };
        
        let mut summary = HashMap::new();
        summary.insert("total_commands".to_string(), serde_json::json!(total));
        summary.insert("successful".to_string(), serde_json::json!(successful));
        summary.insert("failed".to_string(), serde_json::json!(failed));
        summary.insert("average_response_time_ms".to_string(), serde_json::json!(avg_time));
        summary.insert("success_rate".to_string(), serde_json::json!(if total > 0 { successful as f64 / total as f64 * 100.0 } else { 0.0 }));
        
        summary
    }
}

// Macro to wrap command execution with performance tracking
#[macro_export]
macro_rules! track_performance {
    ($tracker:expr, $ctx:expr, $command:expr, $subcommand:expr, $body:expr) => {{
        let command_id = format!("{}_{}_{}_{}", 
            $command, 
            $subcommand.as_deref().unwrap_or("none"),
            $ctx.author().id,
            chrono::Utc::now().timestamp_millis()
        );
        
        $tracker.start_timer(command_id.clone()).await;
        
        let result = $body;
        
        let success = result.is_ok();
        let error = result.as_ref().err().map(|e| e.to_string());
        
        let execution_time = $tracker.end_timer(
            command_id,
            $command.to_string(),
            $subcommand,
            $ctx.author().id.0,
            $ctx.guild_id().map(|id| id.0),
            success,
            error,
        ).await;
        
        if let Some(time) = execution_time {
            tracing::info!(
                "Command {}/{} executed in {:.2}ms", 
                $command, 
                $subcommand.as_deref().unwrap_or("none"),
                time
            );
        }
        
        result
    }};
}