# Slash Command Deployment Guide

This guide explains how to deploy slash commands for your Discord bot globally or to specific guilds.

## 🚀 Quick Start

### Prerequisites
- Rust and Cargo installed
- `DISCORD_TOKEN` environment variable set
- Bot invited to servers with `applications.commands` scope

### Simple Deployment

```bash
# Deploy globally (recommended for production)
make deploy-global

# Deploy to development guild
make deploy-guild GUILD_ID=123456789012345678

# Deploy using environment configuration
make deploy
```

## 📋 Deployment Options

### 1. Global Deployment (Production)

Deploy commands to all servers where your bot is present:

```bash
# Using Makefile
make deploy-global

# Using deployment script
./scripts/deploy.sh --global

# Using Cargo directly
cargo run --bin deploy_commands -- --global
```

**Pros:**
- Commands available in all servers
- Users familiar with commands across servers
- Single deployment for all guilds

**Cons:**
- Takes up to 1 hour to propagate
- Can't test changes quickly

### 2. Guild Deployment (Development)

Deploy commands to specific guild for testing:

```bash
# Using Makefile
make deploy-guild GUILD_ID=123456789012345678

# Using deployment script
./scripts/deploy.sh --guild 123456789012345678

# Using Cargo directly
cargo run --bin deploy_commands -- --guild 123456789012345678
```

**Pros:**
- Available immediately
- Perfect for testing
- Won't affect other servers

**Cons:**
- Only available in specified guild
- Need to deploy to each guild separately

### 3. Environment-Based Deployment

Configure via environment variables:

```bash
# Set environment variables
export DEVELOPMENT_GUILD_ID=123456789012345678
export SLASH_COMMANDS_GLOBAL=false

# Deploy
make deploy
```

## 🔧 Configuration

### Environment Variables

| Variable | Description | Default | Example |
|----------|-------------|---------|---------|
| `DISCORD_TOKEN` | Bot token (required) | - | `NzA4N...` |
| `DEVELOPMENT_GUILD_ID` | Guild for development | none | `123456789012345678` |
| `SLASH_COMMANDS_GLOBAL` | Deploy globally by default | `false` | `true` |
| `AUTO_SYNC_COMMANDS` | Clear existing commands first | `false` | `true` |

### .env File Example

```env
DISCORD_TOKEN=your_bot_token_here
DEVELOPMENT_GUILD_ID=123456789012345678
SLASH_COMMANDS_GLOBAL=false
AUTO_SYNC_COMMANDS=true
COMMAND_PREFIX=!
```

## 🛠️ Deployment Workflow

### Development Workflow

1. **Initial Setup**
   ```bash
   # Set development guild
   export DEVELOPMENT_GUILD_ID=123456789012345678
   
   # Deploy to guild for testing
   make deploy-guild GUILD_ID=123456789012345678
   ```

2. **Testing Changes**
   ```bash
   # Make code changes to commands
   
   # Redeploy to guild (immediate)
   make deploy-guild GUILD_ID=123456789012345678
   
   # Test commands in Discord
   ```

3. **Production Release**
   ```bash
   # Deploy globally when ready
   make deploy-global
   
   # Wait up to 1 hour for propagation
   ```

### Production Workflow

```bash
# 1. Build and test
make build
make test

# 2. Dry run deployment
make deploy-global-dry

# 3. Deploy to production
make deploy-global

# 4. Monitor logs for successful registration
```

## 📊 Available Commands

The deployment system will register these slash commands:

| Command | Description | Options |
|---------|-------------|---------|
| `/ping` | Check bot responsiveness | None |
| `/help` | Show available commands | `command` (optional) |
| `/info` | Get server/user/bot info | `type`, `target` (optional) |

## 🔍 Troubleshooting

### Common Issues

**1. Permission Errors**
```
Error: Missing Access
```
**Solution:** Ensure bot has `applications.commands` scope in target guild.

**2. Token Issues**
```
Error: 401 Unauthorized
```
**Solution:** Check `DISCORD_TOKEN` is correct and bot token (not user token).

**3. Guild Not Found**
```
Error: Unknown Guild
```
**Solution:** Verify guild ID and ensure bot is in the guild.

**4. Rate Limited**
```
Error: 429 Too Many Requests
```
**Solution:** Wait and retry. Avoid rapid redeployments.

### Verification

Check if commands are registered:

```bash
# List all global commands
curl -X GET \
  -H "Authorization: Bot $DISCORD_TOKEN" \
  https://discord.com/api/v10/applications/APPLICATION_ID/commands

# List guild commands
curl -X GET \
  -H "Authorization: Bot $DISCORD_TOKEN" \
  https://discord.com/api/v10/applications/APPLICATION_ID/guilds/GUILD_ID/commands
```

### Debugging

Enable debug logging:

```bash
export DEBUG=true
make deploy-global
```

## 🎯 Best Practices

### Development
- Always test in development guild first
- Use descriptive command names and descriptions
- Test all command options and edge cases
- Verify error handling works correctly

### Production
- Deploy during low-usage hours
- Monitor bot logs during deployment
- Have rollback plan ready
- Document all commands in bot documentation

### Security
- Never commit bot tokens to version control
- Use environment variables for sensitive data
- Rotate tokens regularly
- Monitor for unauthorized deployments

## 🔄 Rollback

To remove all commands:

```bash
# Remove global commands
curl -X PUT \
  -H "Authorization: Bot $DISCORD_TOKEN" \
  -H "Content-Type: application/json" \
  -d '[]' \
  https://discord.com/api/v10/applications/APPLICATION_ID/commands

# Remove guild commands
curl -X PUT \
  -H "Authorization: Bot $DISCORD_TOKEN" \
  -H "Content-Type: application/json" \
  -d '[]' \
  https://discord.com/api/v10/applications/APPLICATION_ID/guilds/GUILD_ID/commands
```

## 📚 Additional Resources

- [Discord API Documentation](https://discord.com/developers/docs/interactions/application-commands)
- [Serenity Documentation](https://docs.rs/serenity/)
- [Bot Permissions Calculator](https://discordapi.com/permissions.html)

## 🆘 Support

If you encounter issues:

1. Check the troubleshooting section above
2. Verify your bot configuration
3. Check Discord API status
4. Review deployment logs
5. Test with a minimal command first