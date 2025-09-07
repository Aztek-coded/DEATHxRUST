#!/bin/bash

# Universal Command Testing System V2
# Enhanced with robust persistence and automatic HTML generation

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

# Test data directory
TEST_DATA_DIR="test_results"
CURRENT_SESSION_ID=""

# Create test data directory if it doesn't exist
mkdir -p "$TEST_DATA_DIR"

# Load test configuration
source .env.test 2>/dev/null || {
    echo -e "${RED}Error: .env.test not found${NC}"
    echo "Creating default .env.test file..."
    cat > .env.test << 'EOF'
# Test Configuration
TEST_GUILD_ID=your_guild_id_here
TEST_CHANNEL_ID=your_channel_id_here
TEST_USER_ID=your_user_id_here
EOF
    echo -e "${YELLOW}Please edit .env.test with your test values${NC}"
    exit 1
}

# Also load main .env for bot token
if [ -f .env ]; then
    source .env
fi

# Function to initialize persistence
init_persistence() {
    echo -e "${CYAN}Initializing persistence layer...${NC}"
    
    # Check if migration is needed
    if [ -f "$TEST_DATA_DIR/universal_test_status.json" ] && [ ! -f "$TEST_DATA_DIR/test_data.db" ]; then
        echo -e "${YELLOW}Migrating existing test data to database...${NC}"
        python3 scripts/test_persistence.py
    fi
    
    # Start new session
    CURRENT_SESSION_ID=$(python3 -c "
from scripts.test_persistence import TestPersistence
p = TestPersistence()
session_id = p.start_session('$TEST_USER_ID', '$TEST_GUILD_ID', '$TEST_CHANNEL_ID')
print(session_id)
")
    
    echo -e "${GREEN}✅ Session started: $CURRENT_SESSION_ID${NC}"
}

# Function to discover commands
discover_commands() {
    echo -e "${CYAN}Discovering available commands...${NC}"
    python3 scripts/command_extractor.py > /dev/null 2>&1
    
    if [ ! -f "$TEST_DATA_DIR/discovered_commands.json" ]; then
        echo -e "${RED}Failed to discover commands${NC}"
        return 1
    fi
    
    # Update command registry in database
    python3 -c "
import json
from scripts.test_persistence import TestPersistence

with open('$TEST_DATA_DIR/discovered_commands.json') as f:
    commands = json.load(f)

p = TestPersistence()
p.update_command_registry(commands)
print(f'✅ Updated registry with {len(commands)} commands')
"
    
    return 0
}

# Function to record test result
record_test_result() {
    local command=$1
    local subcommand=$2
    local status=$3
    local notes=$4
    local execution_time=${5:-0}
    
    python3 -c "
from scripts.test_persistence import TestPersistence

p = TestPersistence()
p.record_test(
    '$CURRENT_SESSION_ID',
    '$command',
    '$subcommand' if '$subcommand' != '_self' else None,
    '$status',
    '''$notes''',
    execution_time_ms=$execution_time
)
"
}

# Function to display enhanced dashboard
show_dashboard() {
    clear
    echo -e "${CYAN}╔═══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║         UNIVERSAL COMMAND TEST STATUS DASHBOARD V2           ║${NC}"
    echo -e "${CYAN}╚═══════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    
    # Get statistics from database
    python3 << 'EOF'
from scripts.test_persistence import TestPersistence
from datetime import datetime

p = TestPersistence()
stats = p.get_statistics()
results = p.get_all_test_results()

# Display statistics
print(f"📊 Overall Statistics:")
print(f"   Total Commands Tested: {stats['total_commands']}")
print(f"   Total Test Runs: {stats['total_tests']}")
print(f"   Tests in Last 24h: {stats['tests_last_24h']}")

# Calculate coverage
status_dist = stats.get('status_distribution', {})
total = sum(status_dist.values())
if total > 0:
    passed = status_dist.get('passed', 0)
    coverage = (passed * 100 // total)
    print(f"   Test Coverage: {passed}/{total} ({coverage}%)")

# Status distribution
print(f"\n📈 Status Distribution:")
icons = {
    "passed": "✅",
    "failed": "❌",
    "partial": "⚠️",
    "untested": "⏳",
    "skipped": "⏭️",
    "rejected": "🚫"
}

for status, count in status_dist.items():
    icon = icons.get(status, "❓")
    print(f"   {icon} {status.capitalize()}: {count}")

print("\n" + "─" * 65)

# Display recent test results
print("\n📋 Recent Test Activity:")
recent_results = sorted(results, key=lambda x: x.get('tested_at', ''), reverse=True)[:10]

for result in recent_results:
    command = result.get('command', '')
    subcommand = result.get('subcommand', '')
    status = result.get('status', 'untested')
    tested_at = result.get('tested_at', '')
    
    if subcommand:
        cmd_display = f"/{command} {subcommand}"
    else:
        cmd_display = f"/{command}"
    
    icon = icons.get(status, "❓")
    
    # Format timestamp
    if tested_at:
        try:
            dt = datetime.fromisoformat(tested_at.replace('Z', '+00:00'))
            tested_at = dt.strftime('%H:%M')
        except:
            pass
    
    print(f"   {icon} {cmd_display:25} [{status:8}] {tested_at}")
EOF
    
    echo ""
    echo "─────────────────────────────────────────────────────────────────"
}

# Function to test a command with timing
test_command_interactive() {
    local command=$1
    local subcommand=$2
    local description=$3
    
    clear
    echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
    if [ "$subcommand" = "_self" ] || [ -z "$subcommand" ]; then
        echo -e "${YELLOW}Testing: /$command${NC}"
        local discord_cmd="/$command"
    else
        echo -e "${YELLOW}Testing: /$command $subcommand${NC}"
        local discord_cmd="/$command $subcommand"
    fi
    echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
    echo ""
    echo -e "Description: $description"
    echo ""
    
    # Get current test status from database
    local current_status=$(python3 -c "
from scripts.test_persistence import TestPersistence
p = TestPersistence()
result = p.get_test_status('$command', '$subcommand' if '$subcommand' != '_self' else None)
print(result.get('status', 'untested'))
")
    
    if [ "$current_status" != "untested" ]; then
        echo -e "${YELLOW}Current Status: $current_status${NC}"
        
        # Show test history
        python3 -c "
from scripts.test_persistence import TestPersistence
import sqlite3

p = TestPersistence()
conn = sqlite3.connect(p.db_path)
cursor = conn.cursor()

cursor.execute('''
    SELECT tested_at, status, notes
    FROM test_results
    WHERE command = ? AND subcommand = ?
    ORDER BY tested_at DESC
    LIMIT 5
''', ('$command', '$subcommand' if '$subcommand' != '_self' else ''))

results = cursor.fetchall()
if results:
    print('\n📜 Test History:')
    for tested_at, status, notes in results:
        print(f'   {tested_at[:16]} - {status} - {notes[:50]}')
conn.close()
"
        echo ""
    fi
    
    # Check if command supports both slash and prefix commands
    local prefix_support=$(python3 -c "
import json
with open('$TEST_DATA_DIR/discovered_commands.json') as f:
    commands = json.load(f)
cmd_info = commands.get('$command', {})
supports_prefix = cmd_info.get('supports_prefix', False)
supports_slash = cmd_info.get('supports_slash', False)
print(f'{supports_prefix}|{supports_slash}')
")
    
    IFS='|' read -r supports_prefix supports_slash <<< "$prefix_support"
    
    echo -e "${BLUE}Test this command in Discord:${NC}"
    
    if [ "$supports_slash" = "True" ]; then
        echo -e "  $discord_cmd"
    fi
    
    if [ "$supports_prefix" = "True" ]; then
        # Get the current prefix (default to !)
        local current_prefix="!"
        if [ "$subcommand" = "_self" ] || [ -z "$subcommand" ]; then
            local prefix_cmd="$current_prefix$command"
        else
            local prefix_cmd="$current_prefix$command $subcommand"
        fi
        echo -e "  $prefix_cmd"
    fi
    echo ""
    echo "─────────────────────────────────────────────────────────────────"
    echo ""
    
    # Start timing
    local start_time=$(python3 -c "import time; print(int(time.time() * 1000))")
    
    # Testing options
    echo "After testing in Discord, select result:"
    echo -e "${GREEN}1)${NC} ✅ Passed - Command works as expected"
    echo -e "${RED}2)${NC} ❌ Failed - Command doesn't work"
    echo -e "${YELLOW}3)${NC} ⚠️  Partial - Some features work, others don't"
    echo -e "${CYAN}4)${NC} ⏭️  Skip - Skip this test for now"
    echo -e "${MAGENTA}5)${NC} 🚫 Reject - Command should not be used"
    echo -e "6) 📝 Add notes without changing status"
    echo -e "0) ← Back to menu"
    echo ""
    
    read -p "Enter your choice (0-6): " choice
    
    # Calculate execution time
    local end_time=$(python3 -c "import time; print(int(time.time() * 1000))")
    local execution_time=$((end_time - start_time))
    
    case $choice in
        1)
            read -p "Notes (optional): " notes
            record_test_result "$command" "$subcommand" "passed" "$notes" "$execution_time"
            echo -e "${GREEN}✅ Test marked as PASSED${NC}"
            ;;
        2)
            read -p "What failed? (required): " notes
            while [ -z "$notes" ]; do
                echo -e "${RED}Please describe what failed:${NC}"
                read -p "Failure reason: " notes
            done
            record_test_result "$command" "$subcommand" "failed" "$notes" "$execution_time"
            echo -e "${RED}❌ Test marked as FAILED${NC}"
            ;;
        3)
            read -p "What partially works? (required): " notes
            while [ -z "$notes" ]; do
                echo -e "${YELLOW}Please describe what partially works:${NC}"
                read -p "Partial reason: " notes
            done
            record_test_result "$command" "$subcommand" "partial" "$notes" "$execution_time"
            echo -e "${YELLOW}⚠️ Test marked as PARTIAL${NC}"
            ;;
        4)
            read -p "Reason for skipping (optional): " notes
            record_test_result "$command" "$subcommand" "skipped" "$notes" "$execution_time"
            echo -e "${CYAN}⏭️ Test SKIPPED${NC}"
            ;;
        5)
            read -p "Rejection reason (required): " notes
            while [ -z "$notes" ]; do
                echo -e "${MAGENTA}Please provide rejection reason:${NC}"
                read -p "Rejection reason: " notes
            done
            record_test_result "$command" "$subcommand" "rejected" "$notes" "$execution_time"
            echo -e "${MAGENTA}🚫 Command REJECTED${NC}"
            ;;
        6)
            read -p "Notes to add: " notes
            if [ ! -z "$notes" ]; then
                record_test_result "$command" "$subcommand" "$current_status" "$notes" 0
                echo -e "${BLUE}📝 Notes added${NC}"
            fi
            ;;
        0)
            return
            ;;
        *)
            echo -e "${RED}Invalid choice${NC}"
            ;;
    esac
    
    echo ""
    read -p "Press Enter to continue..."
}

# Generate comprehensive HTML report
generate_html_report() {
    clear
    echo -e "${CYAN}Generating Comprehensive HTML Report...${NC}"
    
    python3 -c "
from scripts.html_generator import HTMLReportGenerator
from scripts.test_persistence import TestPersistence

p = TestPersistence()
gen = HTMLReportGenerator(p)
report_path = gen.generate_report(title='Universal Command Test Report', auto_open=True)
print(f'✅ Report generated and opened: {report_path}')
"
    
    echo ""
    read -p "Press Enter to continue..."
}

# Backup and restore functions
manage_backups() {
    clear
    echo -e "${CYAN}BACKUP MANAGEMENT${NC}"
    echo "─────────────────────────"
    echo ""
    
    python3 -c "
from scripts.test_persistence import TestPersistence
from pathlib import Path
import os

p = TestPersistence()
backups = sorted(p.backup_dir.glob('*.db.gz'), key=lambda x: x.stat().st_mtime, reverse=True)

print(f'Found {len(backups)} backups:\n')
for i, backup in enumerate(backups[:10], 1):
    size = backup.stat().st_size / 1024
    print(f'{i}) {backup.name} ({size:.1f} KB)')
"
    
    echo ""
    echo "Options:"
    echo "1) Create new backup"
    echo "2) Restore from backup"
    echo "3) Export to JSON"
    echo "4) Create snapshot"
    echo "0) Back to main menu"
    echo ""
    
    read -p "Select option: " choice
    
    case $choice in
        1)
            echo -e "${CYAN}Creating backup...${NC}"
            python3 -c "
from scripts.test_persistence import TestPersistence
p = TestPersistence()
backup_path = p.create_backup('manual')
print(f'✅ Backup created: {backup_path}')
"
            ;;
        2)
            read -p "Enter backup number to restore: " backup_num
            python3 -c "
from scripts.test_persistence import TestPersistence
from pathlib import Path

p = TestPersistence()
backups = sorted(p.backup_dir.glob('*.db.gz'), key=lambda x: x.stat().st_mtime, reverse=True)

try:
    idx = int('$backup_num') - 1
    if 0 <= idx < len(backups):
        if p.restore_backup(str(backups[idx])):
            print(f'✅ Restored from {backups[idx].name}')
        else:
            print('❌ Restore failed')
except:
    print('❌ Invalid selection')
"
            ;;
        3)
            echo -e "${CYAN}Exporting to JSON...${NC}"
            python3 -c "
from scripts.test_persistence import TestPersistence
p = TestPersistence()
export_path = p.export_to_json()
print(f'✅ Exported to: {export_path}')
"
            ;;
        4)
            read -p "Snapshot description: " desc
            python3 -c "
from scripts.test_persistence import TestPersistence
p = TestPersistence()
snapshot_id = p.create_snapshot('$desc')
print(f'✅ Snapshot created with ID: {snapshot_id}')
"
            ;;
    esac
    
    echo ""
    read -p "Press Enter to continue..."
}

# Main testing menu
test_command_menu() {
    local cmd_name=$1
    
    while true; do
        clear
        echo -e "${CYAN}TESTING: /$cmd_name${NC}"
        echo "─────────────────────────"
        
        # Get command info and subcommands
        python3 << EOF
import json
from scripts.test_persistence import TestPersistence

with open('$TEST_DATA_DIR/discovered_commands.json') as f:
    commands = json.load(f)

cmd = commands.get('$cmd_name', {})
if not cmd:
    print("Command not found")
    exit(1)

print(f"Description: {cmd.get('description', 'No description')}")

subcmds = cmd.get('subcommands', {})
if subcmds:
    print(f"\nSubcommands ({len(subcmds)}):")
    
    p = TestPersistence()
    for i, (name, info) in enumerate(sorted(subcmds.items()), 1):
        status_info = p.get_test_status('$cmd_name', name)
        status = status_info.get('status', 'untested')
        
        icons = {
            "passed": "✅",
            "failed": "❌",
            "partial": "⚠️",
            "untested": "⏳",
            "skipped": "⏭️",
            "rejected": "🚫"
        }
        
        icon = icons.get(status, "❓")
        desc = info.get('description', '')[:40]
        print(f"{i}) {icon} {name} - {desc}")
else:
    print("\nThis is a simple command without subcommands")
    print("Press 1 to test it")
EOF
        
        echo ""
        echo "0) Back to main menu"
        echo ""
        read -p "Select option: " choice
        
        if [ "$choice" = "0" ]; then
            return
        fi
        
        # Handle selection
        python3 << EOF
import json

with open('$TEST_DATA_DIR/discovered_commands.json') as f:
    commands = json.load(f)

cmd = commands.get('$cmd_name', {})
subcmds = cmd.get('subcommands', {})

if subcmds:
    # Command with subcommands
    subcmd_list = sorted(subcmds.keys())
    try:
        idx = int('$choice') - 1
        if 0 <= idx < len(subcmd_list):
            selected = subcmd_list[idx]
            desc = subcmds[selected].get('description', '')
            with open('/tmp/test_selection.txt', 'w') as f:
                f.write(f"{selected}|{desc}")
    except:
        pass
else:
    # Simple command
    if '$choice' == '1':
        with open('/tmp/test_selection.txt', 'w') as f:
            f.write(f"_self|{cmd.get('description', '')}")
EOF
        
        if [ -f /tmp/test_selection.txt ]; then
            IFS='|' read -r selected_subcmd description < /tmp/test_selection.txt
            rm /tmp/test_selection.txt
            test_command_interactive "$cmd_name" "$selected_subcmd" "$description"
        else
            echo -e "${RED}Invalid selection${NC}"
            sleep 1
        fi
    done
}

# Main menu
main_menu() {
    while true; do
        show_dashboard
        echo ""
        echo -e "${CYAN}MAIN MENU${NC}"
        echo "─────────────────────────"
        
        # Show available commands
        echo "Commands to test:"
        python3 -c "
import json
try:
    with open('$TEST_DATA_DIR/discovered_commands.json') as f:
        cmds = json.load(f)
    for i, (name, info) in enumerate(sorted(cmds.items()), 1):
        desc = info.get('description', '')[:40]
        subcmd_count = len(info.get('subcommands', {}))
        if subcmd_count > 0:
            print(f'{i}) /{name} - {desc} ({subcmd_count} subcmds)')
        else:
            print(f'{i}) /{name} - {desc}')
except:
    print('No commands discovered')
"
        
        echo ""
        echo "R) 📄 Generate HTML Report"
        echo "B) 💾 Backup Management"
        echo "D) 🔄 Rediscover Commands"
        echo "S) 📊 Show Statistics"
        echo "0) Exit"
        echo ""
        read -p "Select option: " choice
        
        # Handle numeric choices for commands
        if [[ "$choice" =~ ^[0-9]+$ ]] && [ "$choice" != "0" ]; then
            # Get selected command
            selected_cmd=$(python3 -c "
import json
with open('$TEST_DATA_DIR/discovered_commands.json') as f:
    cmds = json.load(f)
cmd_list = sorted(cmds.keys())
try:
    idx = int('$choice') - 1
    if 0 <= idx < len(cmd_list):
        print(cmd_list[idx])
except:
    pass
")
            
            if [ ! -z "$selected_cmd" ]; then
                test_command_menu "$selected_cmd"
            else
                echo -e "${RED}Invalid selection${NC}"
                sleep 1
            fi
        else
            case $choice in
                R|r) generate_html_report ;;
                B|b) manage_backups ;;
                D|d) 
                    discover_commands
                    read -p "Press Enter to continue..."
                    ;;
                S|s)
                    clear
                    python3 -c "
from scripts.test_persistence import TestPersistence
p = TestPersistence()
stats = p.get_statistics()
print('📊 Detailed Statistics:')
print(f'  Total Commands: {stats[\"total_commands\"]}')
print(f'  Total Sessions: {stats[\"total_sessions\"]}')
print(f'  Total Tests: {stats[\"total_tests\"]}')
print(f'  Tests (24h): {stats[\"tests_last_24h\"]}')
print(f'  Avg Execution Time: {stats[\"avg_execution_time_ms\"]:.0f}ms')
print(f'\nStatus Distribution:')
for status, count in stats[\"status_distribution\"].items():
    print(f'  {status}: {count}')
"
                    echo ""
                    read -p "Press Enter to continue..."
                    ;;
                0)
                    # End session
                    python3 -c "
from scripts.test_persistence import TestPersistence
p = TestPersistence()
p.end_session('$CURRENT_SESSION_ID')
"
                    echo -e "${GREEN}Test session ended. Goodbye!${NC}"
                    exit 0
                    ;;
                *)
                    echo -e "${RED}Invalid option${NC}"
                    sleep 1
                    ;;
            esac
        fi
    done
}

# Startup sequence
echo -e "${CYAN}╔═══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║         UNIVERSAL COMMAND TESTING SYSTEM V2                  ║${NC}"
echo -e "${CYAN}║              With Robust Persistence & HTML Reports          ║${NC}"
echo -e "${CYAN}╚═══════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Initialize persistence
init_persistence

# Discover commands on startup
discover_commands
if [ $? -ne 0 ]; then
    echo -e "${RED}Failed to discover commands. Exiting.${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Test System Ready${NC}"
echo -e "  Session: $CURRENT_SESSION_ID"
echo -e "  Guild: $TEST_GUILD_ID"
echo -e "  Channel: $TEST_CHANNEL_ID"
echo -e "  User: $TEST_USER_ID"
echo ""
sleep 2

# Start main menu
main_menu