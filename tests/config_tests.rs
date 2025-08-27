use death_bot::config::Settings;
use std::sync::Mutex;

static TEST_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_settings_creation() {
    let _lock = TEST_MUTEX.lock().unwrap();
    
    // Store original value if it exists
    let original_token = std::env::var("DISCORD_TOKEN").ok();
    let original_debug = std::env::var("DEBUG").ok();
    
    std::env::set_var("DISCORD_TOKEN", "test_token");
    std::env::set_var("DEBUG", "true");
    
    let settings = Settings::from_env().unwrap();
    assert_eq!(settings.discord_token, "test_token");
    assert_eq!(settings.debug_mode, true);
    
    // Restore or remove environment variables
    match original_token {
        Some(val) => std::env::set_var("DISCORD_TOKEN", val),
        None => std::env::remove_var("DISCORD_TOKEN"),
    }
    match original_debug {
        Some(val) => std::env::set_var("DEBUG", val),
        None => std::env::remove_var("DEBUG"),
    }
}

#[test]
fn test_settings_missing_token() {
    let _lock = TEST_MUTEX.lock().unwrap();
    
    // Store original value if it exists
    let original_token = std::env::var("DISCORD_TOKEN").ok();
    
    std::env::remove_var("DISCORD_TOKEN");
    
    let result = Settings::from_env();
    assert!(result.is_err());
    
    // Restore environment variable if it existed
    if let Some(val) = original_token {
        std::env::set_var("DISCORD_TOKEN", val);
    }
}