use death_bot::config::Settings;
use death_bot::testing::integration_runner::{IntegrationTestRunner, TestConfig};
use poise::serenity_prelude::{ChannelId, GuildId, UserId};
use std::env;

/// Run integration tests for boosterrole commands
/// 
/// Usage: cargo test --test test_boosterrole_commands -- --nocapture
/// 
/// Environment variables required:
/// - TEST_BOT_TOKEN: Your bot's token
/// - TEST_GUILD_ID: Guild ID for testing
/// - TEST_CHANNEL_ID: Channel ID for test messages
/// - TEST_USER_ID: User ID with boost status
/// - TEST_ADMIN_ID: User ID with admin permissions
#[tokio::test]
async fn test_boosterrole_extended_commands() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Load test configuration from environment
    let bot_token = env::var("TEST_BOT_TOKEN")
        .expect("TEST_BOT_TOKEN environment variable required");
    
    let guild_id = env::var("TEST_GUILD_ID")
        .expect("TEST_GUILD_ID environment variable required")
        .parse::<u64>()
        .expect("Invalid guild ID");
    
    let channel_id = env::var("TEST_CHANNEL_ID")
        .expect("TEST_CHANNEL_ID environment variable required")
        .parse::<u64>()
        .expect("Invalid channel ID");
    
    let test_user_id = env::var("TEST_USER_ID")
        .expect("TEST_USER_ID environment variable required")
        .parse::<u64>()
        .expect("Invalid user ID");
    
    let admin_user_id = env::var("TEST_ADMIN_ID")
        .expect("TEST_ADMIN_ID environment variable required")
        .parse::<u64>()
        .expect("Invalid admin ID");

    // Create test configuration
    let test_config = TestConfig {
        guild_id: GuildId::new(guild_id),
        test_channel_id: ChannelId::new(channel_id),
        test_role_id: None,
        test_user_id: UserId::new(test_user_id),
        admin_user_id: UserId::new(admin_user_id),
    };

    // Create settings
    let settings = Settings {
        discord_token: bot_token,
        command_prefix: "!".to_string(),
        development_guild_id: Some(guild_id),
        auto_sync_commands: true,
        slash_commands_global: false,
        database_path: "test_bot.db".to_string(),
    };

    // Create and run test runner
    let runner = IntegrationTestRunner::new(settings, test_config);
    
    match runner.run_all_tests().await {
        Ok(report) => {
            println!("{}", report.generate_report());
            
            // Assert all tests passed
            assert_eq!(
                report.failed, 0,
                "Integration tests failed: {} failures",
                report.failed
            );
        }
        Err(e) => {
            panic!("Failed to run integration tests: {}", e);
        }
    }
}

/// Test individual cleanup command
#[tokio::test]
async fn test_cleanup_command_individually() {
    use death_bot::testing::boosterrole_test_suite::BoosterroleTestSuite;
    
    let bot_token = env::var("TEST_BOT_TOKEN")
        .unwrap_or_else(|_| "test_token".to_string());
    
    let test_guild_id = env::var("TEST_GUILD_ID")
        .unwrap_or_else(|_| "1234567890".to_string())
        .parse::<u64>()
        .unwrap_or(1234567890);
    
    let test_channel_id = env::var("TEST_CHANNEL_ID")
        .unwrap_or_else(|_| "9876543210".to_string())
        .parse::<u64>()
        .unwrap_or(9876543210);
    
    let test_user_id = env::var("TEST_USER_ID")
        .unwrap_or_else(|_| "1111111111".to_string())
        .parse::<u64>()
        .unwrap_or(1111111111);

    let suite = BoosterroleTestSuite::new(
        bot_token,
        test_guild_id,
        test_channel_id,
        test_user_id,
    );

    let results = suite.run_all_tests().await;
    
    println!("Test Results: {}", results.summary());
    
    // Print detailed results
    for test in &results.details {
        println!("  {} - {}: {}", 
            match test.status {
                death_bot::testing::boosterrole_test_suite::TestStatus::Passed => "✅",
                death_bot::testing::boosterrole_test_suite::TestStatus::Failed => "❌",
                death_bot::testing::boosterrole_test_suite::TestStatus::Skipped => "⏭️",
            },
            test.name,
            test.message
        );
    }
}

/// Mock test for CI/CD environments without Discord access
#[test]
fn test_command_structure() {
    // This test verifies that commands compile correctly
    // It doesn't require Discord API access
    
    // Test that all command modules exist
    assert!(std::path::Path::new("src/commands/boosterrole/cleanup.rs").exists());
    assert!(std::path::Path::new("src/commands/boosterrole/limit.rs").exists());
    assert!(std::path::Path::new("src/commands/boosterrole/rename.rs").exists());
    assert!(std::path::Path::new("src/commands/boosterrole/award.rs").exists());
    
    println!("✅ All boosterrole command files exist");
}