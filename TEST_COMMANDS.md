# Boosterrole Extended Management - Enhanced Testing Guide

## Your Test Configuration
- **Guild ID**: 1410101805626425346
- **Channel ID**: 1410101806637387888
- **Your User ID**: 758053782243377334

## 🚀 Quick Start Testing

### 1. Start your bot
```bash
cargo run --release
```

### 2. Test Commands in Discord

#### Admin Commands (You have permission for these)

**Cleanup Command** - Remove orphaned roles
```
/boosterrole cleanup dry_run:true
/boosterrole cleanup
```

**Limit Command** - Set max roles allowed
```
/boosterrole limit
/boosterrole limit max_roles:10
/boosterrole limit max_roles:0
```

**Award Command** - Set role for new boosters
```
/boosterrole award view
/boosterrole award set role:@RoleName
/boosterrole award unset
```

#### Booster Commands (Requires boost status)

**Rename Command** - Rename your role
```
/boosterrole rename new_name:MyCoolRole
```
*Note: 60-minute cooldown between renames*

## 📊 Enhanced Testing System

### Performance-Tracked Manual Testing

Run the comprehensive testing script with performance metrics:
```bash
./scripts/comprehensive_tester.sh
```

**Features:**
- Real-time performance tracking for each test
- Automatic regression detection
- Visual test status dashboard
- Performance baseline comparisons
- Execution time measurements
- Resource usage monitoring

### 🎯 Automated Test Runner

Execute all tests programmatically with detailed metrics:
```bash
# Run all test suites
python3 scripts/automated_test_runner.py

# Run tests in parallel for faster execution
python3 scripts/automated_test_runner.py --parallel

# Run specific test suite only
python3 scripts/automated_test_runner.py --suite cleanup

# Save current performance as baseline
python3 scripts/automated_test_runner.py --save-baseline
```

### 📈 Enhanced HTML Dashboard

Generate an interactive dashboard with performance visualizations:
```bash
# Generate dashboard
python3 scripts/dashboard_generator.py

# Dashboard includes:
# - Performance trends charts
# - Test status distribution
# - Command response time graphs
# - Resource usage visualization
# - Filterable test results
# - Real-time metrics cards
```

### 🔄 Real-Time Dashboard Server

Start WebSocket server for live dashboard updates:
```bash
# Start server (default: ws://localhost:8765)
python3 scripts/dashboard_server.py

# Custom configuration
python3 scripts/dashboard_server.py --host 0.0.0.0 --port 8080
```

## 📊 Performance Metrics Collection

### Collecting Performance Data

Performance metrics are automatically collected during testing:

```bash
# View performance report
python3 scripts/performance_collector.py report

# Set performance baseline
python3 scripts/performance_collector.py baseline

# Track specific command
python3 scripts/performance_collector.py start cleanup dry_run
python3 scripts/performance_collector.py end cleanup dry_run completed
```

### Metrics Tracked
- **Execution Time**: Command response time in milliseconds
- **CPU Usage**: Processor utilization delta
- **Memory Usage**: RAM consumption changes
- **Disk I/O**: Read/write operations
- **Network I/O**: Data sent/received
- **Error Rates**: Failure frequency tracking

### Performance Baselines

Configure performance thresholds in `config/performance_baselines.json`:
```json
{
  "thresholds": {
    "response_time": {
      "excellent": 200,
      "good": 500,
      "acceptable": 1000,
      "warning": 2000,
      "critical": 5000
    }
  }
}
```

## 🧪 Testing Scenarios

### Scenario 1: Performance Regression Testing
1. Run initial tests to establish baseline:
   ```bash
   python3 scripts/automated_test_runner.py --save-baseline
   ```
2. Make code changes
3. Run tests again to check for regressions:
   ```bash
   python3 scripts/automated_test_runner.py
   ```
4. Review dashboard for performance warnings

### Scenario 2: Load Testing
1. Configure parallel execution in test config
2. Run automated tests with increased load:
   ```bash
   python3 scripts/automated_test_runner.py --parallel
   ```
3. Monitor resource usage in real-time dashboard

### Scenario 3: Continuous Monitoring
1. Start WebSocket server:
   ```bash
   python3 scripts/dashboard_server.py &
   ```
2. Open dashboard in browser
3. Run tests while monitoring live updates
4. Track performance trends over time

### Scenario 4: Test Cleanup
1. Create a booster role for a test user
2. Remove their boost status
3. Run `/boosterrole cleanup dry_run:true` to preview
4. Run `/boosterrole cleanup` to actually clean
5. Check performance metrics in dashboard

### Scenario 5: Test Limits
1. Set limit: `/boosterrole limit max_roles:2`
2. Try creating 3 booster roles
3. Third should fail with limit message
4. Review execution times for each attempt

### Scenario 6: Test Rename with Performance
1. Create a booster role
2. Rename it: `/boosterrole rename new_name:TestRole1`
3. Monitor response time
4. Try immediate second rename (should fail - cooldown)
5. Check if failure response is faster than success

## 📁 Test Data Management

### Test Results Location
```bash
test_results/
├── test_status.json           # Current test status
├── performance_metrics.json   # Performance data
├── performance_baselines.json # Baseline metrics
├── test_history.log           # Test execution history
├── dashboard_*.html           # Generated dashboards
└── automated_test_results_*.json # Automated test outputs
```

### Export/Import Test Data
```bash
# Export current test status
cp test_results/test_status.json test_results/export_$(date +%Y%m%d).json

# Import previous test status
cp test_results/export_20250907.json test_results/test_status.json
```

### Clean Test Data
```bash
# Reset all test data (WARNING: removes history)
rm -rf test_results/*
./scripts/comprehensive_tester.sh  # Will recreate with defaults
```

## 🔧 Troubleshooting

### Bot not responding to commands?
1. Check bot is online in Discord
2. Check bot has proper permissions in the server
3. Verify slash commands are registered:
   ```bash
   make deploy-guild GUILD_ID=1410101805626425346
   ```

### Performance metrics not collecting?
1. Ensure Python dependencies are installed:
   ```bash
   pip install psutil websockets
   ```
2. Check Python version (requires 3.7+):
   ```bash
   python3 --version
   ```

### Dashboard not opening?
1. Check if HTML file was generated:
   ```bash
   ls test_results/dashboard_*.html
   ```
2. Open manually in browser if auto-open fails
3. Ensure JavaScript is enabled for Chart.js

### WebSocket connection failing?
1. Check if port is available:
   ```bash
   lsof -i :8765
   ```
2. Try different port:
   ```bash
   python3 scripts/dashboard_server.py --port 8080
   ```

### Commands not showing in Discord?
```bash
# Deploy commands to your guild
make deploy-guild GUILD_ID=1410101805626425346

# Or globally (takes up to 1 hour)
make deploy-global
```

### Database issues?
```bash
# Check database exists
ls -la bot_data.db

# Reset database (WARNING: deletes all data)
rm bot_data.db
cargo run --release
```

## 📊 Dashboard Features

### Real-Time Metrics Cards
- **Total Tests**: Number of tests executed
- **Pass Rate**: Percentage of successful tests
- **Avg Response Time**: Mean execution time
- **Test Coverage**: Percentage of commands tested
- **Performance Score**: Overall system performance (0-100)
- **Regressions**: Count of performance degradations

### Interactive Charts
1. **Performance Trends**: Line chart showing response times over time
2. **Status Distribution**: Doughnut chart of test results
3. **Command Response Times**: Bar chart comparing command speeds
4. **Resource Usage**: Radar chart of system metrics

### Filterable Results Table
- Filter by status (passed, failed, partial, etc.)
- Sort by execution time
- View detailed notes for each test
- Performance indicators (⚡ fast, ⚠️ warning, 🐌 slow)

## 🚀 Advanced Usage

### Custom Test Configuration

Create a custom test configuration file:
```json
{
  "test_suites": {
    "critical": [
      {"subcommand": "cleanup", "args": ["dry_run:true"], "expected": "success"}
    ]
  },
  "execution": {
    "parallel": true,
    "timeout_seconds": 60,
    "retry_on_failure": true
  }
}
```

Run with custom config:
```bash
python3 scripts/automated_test_runner.py --config custom_tests.json
```

### Performance Baseline Management

```bash
# Generate performance report
python3 scripts/performance_collector.py report

# Set new baseline from current metrics
python3 scripts/performance_collector.py baseline

# Set baseline for specific command
python3 scripts/performance_collector.py baseline cleanup dry_run
```

### Continuous Integration

Add to CI/CD pipeline:
```yaml
- name: Run automated tests
  run: |
    python3 scripts/automated_test_runner.py
    if [ $? -ne 0 ]; then
      echo "Tests failed"
      exit 1
    fi
```

## 📝 Notes

- Performance metrics are retained for 30 days by default
- Dashboard auto-refreshes every 30 seconds when server is running
- Regression threshold is 20% by default (configurable)
- All times are in UTC for consistency
- WebSocket server supports multiple concurrent clients