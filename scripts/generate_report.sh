#!/bin/bash

# Quick HTML Report Generator for Boosterrole Tests

# Colors
GREEN='\033[0;32m'
CYAN='\033[0;36m'
NC='\033[0m'

# Test data directory
TEST_DATA_DIR="test_results"

# Check if test data exists
if [ ! -f "$TEST_DATA_DIR/test_status.json" ]; then
    echo "No test data found. Run ./scripts/comprehensive_tester.sh first to test some commands."
    exit 1
fi

echo -e "${CYAN}Generating HTML Test Report...${NC}"

# Generate report filename with timestamp
REPORT_FILE="$TEST_DATA_DIR/test_report_$(date +%Y%m%d_%H%M%S).html"

# Create HTML report
cat > "$REPORT_FILE" << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Boosterrole Test Report</title>
    <style>
        body { font-family: 'Segoe UI', Arial, sans-serif; margin: 20px; background: #1a1a1a; color: #e0e0e0; }
        .container { max-width: 1200px; margin: 0 auto; }
        h1 { color: #7289da; text-align: center; padding: 20px; background: #2f3136; border-radius: 10px; }
        h2 { color: #5865f2; margin-top: 30px; border-bottom: 2px solid #5865f2; padding-bottom: 10px; }
        .stats { background: #2f3136; padding: 20px; border-radius: 10px; margin: 20px 0; display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 15px; }
        .stat-card { background: #36393f; padding: 15px; border-radius: 8px; text-align: center; }
        .stat-number { font-size: 2em; font-weight: bold; }
        .passed { color: #43b581; }
        .failed { color: #f04747; }
        .partial { color: #faa61a; }
        .untested { color: #747f8d; }
        .skipped { color: #5865f2; }
        .rejected { color: #e91e63; }
        table { width: 100%; border-collapse: collapse; margin: 20px 0; background: #2f3136; border-radius: 10px; overflow: hidden; }
        th { background: #202225; padding: 12px; text-align: left; font-weight: 600; }
        td { padding: 12px; border-bottom: 1px solid #40444b; }
        tr:hover { background: #36393f; }
        .notes { font-size: 0.9em; color: #b9bbbe; font-style: italic; }
        .timestamp { font-size: 0.85em; color: #72767d; }
        .progress-bar { width: 100%; height: 30px; background: #202225; border-radius: 15px; overflow: hidden; margin: 20px 0; }
        .progress-fill { height: 100%; background: linear-gradient(90deg, #43b581, #5865f2); transition: width 0.3s; }
        .legend { display: flex; gap: 20px; flex-wrap: wrap; margin: 20px 0; }
        .legend-item { display: flex; align-items: center; gap: 5px; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🧪 Boosterrole Extended Management - Test Report</h1>
EOF

# Add test data using Python
python3 << 'PYTHON' >> "$REPORT_FILE"
import json
from datetime import datetime

try:
    with open('test_results/test_status.json', 'r') as f:
        data = json.load(f)
except:
    print("<p>Error: Could not load test data</p>")
    exit()

# Calculate statistics
total_tests = 0
status_counts = {"passed": 0, "failed": 0, "partial": 0, "untested": 0, "skipped": 0, "rejected": 0}
commands_tested = set()

for command, subcmds in data.items():
    for subcommand, info in subcmds.items():
        total_tests += 1
        status = info.get('status', 'untested')
        if status in status_counts:
            status_counts[status] += 1
        if status != 'untested':
            commands_tested.add(command)

coverage = (status_counts['passed'] * 100 // total_tests) if total_tests > 0 else 0
tested_count = total_tests - status_counts['untested']

# Progress bar
print(f'<div class="progress-bar">')
print(f'<div class="progress-fill" style="width: {coverage}%"></div>')
print(f'</div>')
print(f'<p style="text-align: center; font-size: 1.2em;">Test Coverage: {coverage}% ({status_counts["passed"]}/{total_tests} tests passed)</p>')

# Statistics cards
print('<div class="stats">')
print(f'<div class="stat-card"><div class="stat-number passed">{status_counts["passed"]}</div><div>✅ Passed</div></div>')
print(f'<div class="stat-card"><div class="stat-number failed">{status_counts["failed"]}</div><div>❌ Failed</div></div>')
print(f'<div class="stat-card"><div class="stat-number partial">{status_counts["partial"]}</div><div>⚠️ Partial</div></div>')
print(f'<div class="stat-card"><div class="stat-number untested">{status_counts["untested"]}</div><div>⏳ Untested</div></div>')
print(f'<div class="stat-card"><div class="stat-number skipped">{status_counts["skipped"]}</div><div>⏭️ Skipped</div></div>')
print(f'<div class="stat-card"><div class="stat-number rejected">{status_counts["rejected"]}</div><div>🚫 Rejected</div></div>')
print('</div>')

# Metadata
print('<div style="background: #2f3136; padding: 15px; border-radius: 10px; margin: 20px 0;">')
print(f'<p>📅 Report Generated: {datetime.now().strftime("%Y-%m-%d %H:%M:%S")}</p>')
print(f'<p>📊 Total Tests: {total_tests}</p>')
print(f'<p>✔️ Tests Executed: {tested_count}</p>')
print(f'<p>📁 Commands Tested: {len(commands_tested)}/4</p>')
print('</div>')

# Legend
print('<div class="legend">')
print('<div class="legend-item"><span class="passed">✅</span> Passed: Command works as expected</div>')
print('<div class="legend-item"><span class="failed">❌</span> Failed: Command does not work</div>')
print('<div class="legend-item"><span class="partial">⚠️</span> Partial: Some features work</div>')
print('<div class="legend-item"><span class="untested">⏳</span> Untested: Not yet tested</div>')
print('<div class="legend-item"><span class="skipped">⏭️</span> Skipped: Test skipped</div>')
print('<div class="legend-item"><span class="rejected">🚫</span> Rejected: Should not be used</div>')
print('</div>')

# Detailed results by command
print('<h2>📋 Detailed Test Results</h2>')

for command in data.keys():
    subcmds = data[command]
    print(f'<h3 style="color: #7289da;">/{command}</h3>')
    print('<table>')
    print('<tr><th>Subcommand</th><th>Status</th><th>Last Tested</th><th>Notes</th></tr>')
    
    for subcommand, info in subcmds.items():
        status = info.get('status', 'untested')
        last_tested = info.get('last_tested', 'Never')
        notes = info.get('notes', '')
        
        if last_tested != 'Never' and last_tested:
            try:
                dt = datetime.fromisoformat(last_tested.replace('Z', '+00:00'))
                last_tested = dt.strftime('%Y-%m-%d %H:%M')
            except:
                pass
        
        status_icon = {
            "passed": "✅",
            "failed": "❌",
            "partial": "⚠️",
            "untested": "⏳",
            "skipped": "⏭️",
            "rejected": "🚫"
        }.get(status, "❓")
        
        print(f'<tr>')
        print(f'<td><strong>{subcommand}</strong></td>')
        print(f'<td class="{status}">{status_icon} {status.upper()}</td>')
        print(f'<td class="timestamp">{last_tested}</td>')
        print(f'<td class="notes">{notes if notes else "-"}</td>')
        print(f'</tr>')
    
    print('</table>')

# Failed tests summary
failed_tests = []
for command, subcmds in data.items():
    for subcommand, info in subcmds.items():
        if info.get('status') == 'failed':
            failed_tests.append((command, subcommand, info.get('notes', '')))

if failed_tests:
    print('<h2>❌ Failed Tests Summary</h2>')
    print('<div style="background: #2f3136; padding: 20px; border-radius: 10px; border-left: 4px solid #f04747;">')
    for cmd, subcmd, notes in failed_tests:
        print(f'<p><strong>{cmd}/{subcmd}</strong>: {notes}</p>')
    print('</div>')

# Rejected commands
rejected_tests = []
for command, subcmds in data.items():
    for subcommand, info in subcmds.items():
        if info.get('status') == 'rejected':
            rejected_tests.append((command, subcommand, info.get('notes', '')))

if rejected_tests:
    print('<h2>🚫 Rejected Commands</h2>')
    print('<div style="background: #2f3136; padding: 20px; border-radius: 10px; border-left: 4px solid #e91e63;">')
    for cmd, subcmd, notes in rejected_tests:
        print(f'<p><strong>{cmd}/{subcmd}</strong>: {notes}</p>')
    print('</div>')
PYTHON

# Close HTML
echo '</div></body></html>' >> "$REPORT_FILE"

echo -e "${GREEN}✅ HTML Report Generated Successfully!${NC}"
echo ""
echo "📄 Report Location:"
echo "   $REPORT_FILE"
echo ""
echo "Open in browser? (y/n)"
read -p "> " choice

if [ "$choice" = "y" ] || [ "$choice" = "Y" ]; then
    # Try to open in browser (works on macOS and Linux)
    if command -v open &> /dev/null; then
        open "$REPORT_FILE"
    elif command -v xdg-open &> /dev/null; then
        xdg-open "$REPORT_FILE"
    else
        echo "Please open manually: $REPORT_FILE"
    fi
fi