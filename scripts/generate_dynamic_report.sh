#!/bin/bash

# Dynamic HTML Report Generator with Fixed UTF-8 Encoding
# Generates beautiful HTML reports from dynamic test data

# Set UTF-8 encoding
export LANG=en_US.UTF-8
export LC_ALL=en_US.UTF-8

# Colors
GREEN='\033[0;32m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Paths
CONFIG_FILE="test_configs/commands.json"
TEST_DATA_DIR="test_results"
TEST_STATUS_FILE="$TEST_DATA_DIR/test_status_dynamic.json"
REJECTION_LOG="$TEST_DATA_DIR/rejections_dynamic.log"

# Check if test data exists
if [ ! -f "$TEST_STATUS_FILE" ]; then
    echo -e "${YELLOW}No test data found. Run ./scripts/dynamic_tester.sh first.${NC}"
    
    # Initialize empty file
    echo "{}" > "$TEST_STATUS_FILE"
fi

if [ ! -f "$CONFIG_FILE" ]; then
    echo -e "${YELLOW}Warning: Command configuration not found${NC}"
fi

echo -e "${CYAN}Generating Dynamic HTML Test Report...${NC}"

# Generate report filename with timestamp
REPORT_FILE="$TEST_DATA_DIR/test_report_$(date +%Y%m%d_%H%M%S).html"

# Create HTML report with proper UTF-8 encoding
cat > "$REPORT_FILE" << 'HTML_START'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
    <title>Discord Bot Test Report</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Helvetica', 'Arial', sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            padding: 20px;
        }
        
        .container {
            max-width: 1400px;
            margin: 0 auto;
            background: #1a1a2e;
            border-radius: 20px;
            box-shadow: 0 20px 60px rgba(0,0,0,0.3);
            overflow: hidden;
        }
        
        .header {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            padding: 40px;
            text-align: center;
            color: white;
        }
        
        .header h1 {
            font-size: 2.5em;
            margin-bottom: 10px;
            text-shadow: 2px 2px 4px rgba(0,0,0,0.2);
        }
        
        .header .subtitle {
            font-size: 1.1em;
            opacity: 0.9;
        }
        
        .content {
            padding: 40px;
            color: #e0e0e0;
        }
        
        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin-bottom: 40px;
        }
        
        .stat-card {
            background: #16213e;
            padding: 25px;
            border-radius: 15px;
            text-align: center;
            transition: transform 0.3s, box-shadow 0.3s;
            border: 1px solid #2a2a4a;
        }
        
        .stat-card:hover {
            transform: translateY(-5px);
            box-shadow: 0 10px 30px rgba(102, 126, 234, 0.2);
        }
        
        .stat-number {
            font-size: 2.5em;
            font-weight: bold;
            margin-bottom: 5px;
        }
        
        .stat-label {
            font-size: 0.9em;
            color: #b0b0b0;
            text-transform: uppercase;
            letter-spacing: 1px;
        }
        
        .passed { color: #4ade80; }
        .failed { color: #f87171; }
        .partial { color: #fbbf24; }
        .untested { color: #9ca3af; }
        .skipped { color: #60a5fa; }
        .rejected { color: #f472b6; }
        
        .progress-container {
            background: #16213e;
            border-radius: 20px;
            padding: 30px;
            margin-bottom: 40px;
            border: 1px solid #2a2a4a;
        }
        
        .progress-bar {
            width: 100%;
            height: 40px;
            background: #0f172a;
            border-radius: 20px;
            overflow: hidden;
            position: relative;
        }
        
        .progress-fill {
            height: 100%;
            background: linear-gradient(90deg, #4ade80, #667eea);
            transition: width 1s ease;
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
            font-weight: bold;
            text-shadow: 1px 1px 2px rgba(0,0,0,0.3);
        }
        
        .section {
            background: #16213e;
            border-radius: 15px;
            padding: 30px;
            margin-bottom: 30px;
            border: 1px solid #2a2a4a;
        }
        
        .section h2 {
            color: #667eea;
            margin-bottom: 20px;
            font-size: 1.8em;
            border-bottom: 2px solid #2a2a4a;
            padding-bottom: 10px;
        }
        
        .command-group {
            background: #0f172a;
            border-radius: 10px;
            padding: 20px;
            margin-bottom: 20px;
        }
        
        .command-group h3 {
            color: #60a5fa;
            margin-bottom: 15px;
            font-size: 1.3em;
        }
        
        .test-table {
            width: 100%;
            border-collapse: collapse;
        }
        
        .test-table th {
            background: #1e293b;
            padding: 12px;
            text-align: left;
            color: #94a3b8;
            font-weight: 600;
            text-transform: uppercase;
            font-size: 0.85em;
            letter-spacing: 1px;
        }
        
        .test-table td {
            padding: 12px;
            border-bottom: 1px solid #1e293b;
            color: #e2e8f0;
        }
        
        .test-table tr:hover {
            background: #1e293b;
        }
        
        .status-badge {
            display: inline-block;
            padding: 4px 12px;
            border-radius: 20px;
            font-size: 0.85em;
            font-weight: 600;
            text-transform: uppercase;
        }
        
        .status-badge.passed { background: rgba(74, 222, 128, 0.2); color: #4ade80; }
        .status-badge.failed { background: rgba(248, 113, 113, 0.2); color: #f87171; }
        .status-badge.partial { background: rgba(251, 191, 36, 0.2); color: #fbbf24; }
        .status-badge.untested { background: rgba(156, 163, 175, 0.2); color: #9ca3af; }
        .status-badge.skipped { background: rgba(96, 165, 250, 0.2); color: #60a5fa; }
        .status-badge.rejected { background: rgba(244, 114, 182, 0.2); color: #f472b6; }
        
        .notes {
            font-size: 0.9em;
            color: #94a3b8;
            font-style: italic;
        }
        
        .timestamp {
            font-size: 0.85em;
            color: #64748b;
        }
        
        .legend {
            display: flex;
            flex-wrap: wrap;
            gap: 20px;
            padding: 20px;
            background: #0f172a;
            border-radius: 10px;
            margin-bottom: 30px;
        }
        
        .legend-item {
            display: flex;
            align-items: center;
            gap: 8px;
            font-size: 0.9em;
        }
        
        .footer {
            text-align: center;
            padding: 30px;
            color: #64748b;
            background: #0f172a;
            font-size: 0.9em;
        }
        
        @keyframes fadeIn {
            from { opacity: 0; transform: translateY(20px); }
            to { opacity: 1; transform: translateY(0); }
        }
        
        .fade-in {
            animation: fadeIn 0.6s ease forwards;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🧪 Discord Bot Test Report</h1>
            <div class="subtitle">Comprehensive Command Testing Results</div>
        </div>
        
        <div class="content">
HTML_START

# Add dynamic content using Python
python3 << 'PYTHON_SCRIPT' >> "$REPORT_FILE"
import json
import sys
from datetime import datetime
import html

# Set UTF-8 encoding
sys.stdout.reconfigure(encoding='utf-8')

# Load configuration
try:
    with open('test_configs/commands.json', 'r', encoding='utf-8') as f:
        config = json.load(f)
except:
    config = {"test_suites": {}}

# Load test status
try:
    with open('test_results/test_status_dynamic.json', 'r', encoding='utf-8') as f:
        status_data = json.load(f)
except:
    status_data = {}

# Calculate statistics
total_tests = 0
status_counts = {"passed": 0, "failed": 0, "partial": 0, "untested": 0, "skipped": 0, "rejected": 0}
suite_stats = {}

# Count all test cases
for suite_id, suite in config.get('test_suites', {}).items():
    suite_stats[suite_id] = {"total": 0, "passed": 0, "failed": 0}
    for cmd_id, cmd_data in suite.get('commands', {}).items():
        for test_case in cmd_data.get('test_cases', []):
            total_tests += 1
            suite_stats[suite_id]["total"] += 1
            test_status = status_data.get(suite_id, {}).get(cmd_id, {}).get(test_case['id'], {}).get('status', 'untested')
            if test_status in status_counts:
                status_counts[test_status] += 1
                if test_status == 'passed':
                    suite_stats[suite_id]["passed"] += 1
                elif test_status == 'failed':
                    suite_stats[suite_id]["failed"] += 1

coverage = (status_counts['passed'] * 100 // total_tests) if total_tests > 0 else 0

# Generate timestamp
timestamp = datetime.now().strftime('%Y-%m-%d %H:%M:%S')

# Statistics cards
print('<div class="stats-grid fade-in">')
print(f'<div class="stat-card"><div class="stat-number passed">{status_counts["passed"]}</div><div class="stat-label">✅ Passed</div></div>')
print(f'<div class="stat-card"><div class="stat-number failed">{status_counts["failed"]}</div><div class="stat-label">❌ Failed</div></div>')
print(f'<div class="stat-card"><div class="stat-number partial">{status_counts["partial"]}</div><div class="stat-label">⚠️ Partial</div></div>')
print(f'<div class="stat-card"><div class="stat-number untested">{status_counts["untested"]}</div><div class="stat-label">⏳ Untested</div></div>')
print(f'<div class="stat-card"><div class="stat-number skipped">{status_counts["skipped"]}</div><div class="stat-label">⏭️ Skipped</div></div>')
print(f'<div class="stat-card"><div class="stat-number rejected">{status_counts["rejected"]}</div><div class="stat-label">🚫 Rejected</div></div>')
print('</div>')

# Progress bar
print('<div class="progress-container fade-in">')
print('<h3 style="margin-bottom: 15px; color: #94a3b8;">Overall Test Coverage</h3>')
print('<div class="progress-bar">')
print(f'<div class="progress-fill" style="width: {coverage}%">{coverage}% Complete ({status_counts["passed"]}/{total_tests} tests)</div>')
print('</div>')
print('</div>')

# Legend
print('<div class="legend fade-in">')
print('<div class="legend-item"><span class="status-badge passed">✅ PASSED</span> Command works as expected</div>')
print('<div class="legend-item"><span class="status-badge failed">❌ FAILED</span> Command does not work</div>')
print('<div class="legend-item"><span class="status-badge partial">⚠️ PARTIAL</span> Some features work</div>')
print('<div class="legend-item"><span class="status-badge untested">⏳ UNTESTED</span> Not yet tested</div>')
print('<div class="legend-item"><span class="status-badge skipped">⏭️ SKIPPED</span> Test skipped</div>')
print('<div class="legend-item"><span class="status-badge rejected">🚫 REJECTED</span> Should not be used</div>')
print('</div>')

# Detailed results by suite
for suite_id, suite in config.get('test_suites', {}).items():
    suite_name = html.escape(suite.get('name', suite_id))
    suite_desc = html.escape(suite.get('description', ''))
    
    print(f'<div class="section fade-in">')
    print(f'<h2>📦 {suite_name}</h2>')
    if suite_desc:
        print(f'<p style="color: #94a3b8; margin-bottom: 20px;">{suite_desc}</p>')
    
    # Suite statistics
    if suite_id in suite_stats:
        stats = suite_stats[suite_id]
        suite_coverage = (stats["passed"] * 100 // stats["total"]) if stats["total"] > 0 else 0
        print(f'<div style="margin-bottom: 20px; padding: 15px; background: #0f172a; border-radius: 10px;">')
        print(f'<span style="color: #94a3b8;">Suite Coverage: </span>')
        print(f'<strong style="color: #4ade80;">{suite_coverage}%</strong> ')
        print(f'({stats["passed"]}/{stats["total"]} tests passed)')
        print(f'</div>')
    
    # Commands in suite
    for cmd_id, cmd_data in suite.get('commands', {}).items():
        cmd_desc = html.escape(cmd_data.get('description', ''))
        
        print(f'<div class="command-group">')
        print(f'<h3>/{html.escape(cmd_id)}</h3>')
        if cmd_desc:
            print(f'<p style="color: #94a3b8; margin-bottom: 15px;">{cmd_desc}</p>')
        
        # Test cases table
        print('<table class="test-table">')
        print('<thead><tr>')
        print('<th>Test Name</th>')
        print('<th>Status</th>')
        print('<th>Last Tested</th>')
        print('<th>Notes</th>')
        print('</tr></thead>')
        print('<tbody>')
        
        for test_case in cmd_data.get('test_cases', []):
            test_id = test_case['id']
            test_name = html.escape(test_case.get('name', test_id))
            test_desc = html.escape(test_case.get('description', ''))
            
            test_status_data = status_data.get(suite_id, {}).get(cmd_id, {}).get(test_id, {})
            status = test_status_data.get('status', 'untested')
            last_tested = test_status_data.get('last_tested', 'Never')
            notes = html.escape(test_status_data.get('notes', ''))
            
            # Format timestamp
            if last_tested != 'Never':
                try:
                    dt = datetime.fromisoformat(last_tested.replace('Z', '+00:00'))
                    last_tested = dt.strftime('%Y-%m-%d %H:%M')
                except:
                    pass
            
            # Status icon
            status_icons = {
                "passed": "✅", "failed": "❌", "partial": "⚠️",
                "untested": "⏳", "skipped": "⏭️", "rejected": "🚫"
            }
            icon = status_icons.get(status, "❓")
            
            print('<tr>')
            print(f'<td>')
            print(f'<strong>{test_name}</strong>')
            if test_desc:
                print(f'<br><span style="font-size: 0.85em; color: #64748b;">{test_desc}</span>')
            print(f'</td>')
            print(f'<td><span class="status-badge {status}">{icon} {status.upper()}</span></td>')
            print(f'<td class="timestamp">{last_tested}</td>')
            print(f'<td class="notes">{notes if notes else "-"}</td>')
            print('</tr>')
        
        print('</tbody>')
        print('</table>')
        print('</div>')
    
    print('</div>')

# Failed tests summary
failed_tests = []
rejected_tests = []

for suite_id, suite in config.get('test_suites', {}).items():
    for cmd_id, cmd_data in suite.get('commands', {}).items():
        for test_case in cmd_data.get('test_cases', []):
            test_id = test_case['id']
            test_status_data = status_data.get(suite_id, {}).get(cmd_id, {}).get(test_id, {})
            status = test_status_data.get('status', 'untested')
            
            if status == 'failed':
                failed_tests.append({
                    'suite': suite.get('name', suite_id),
                    'command': cmd_id,
                    'test': test_case.get('name', test_id),
                    'notes': test_status_data.get('notes', '')
                })
            elif status == 'rejected':
                rejected_tests.append({
                    'suite': suite.get('name', suite_id),
                    'command': cmd_id,
                    'test': test_case.get('name', test_id),
                    'notes': test_status_data.get('notes', '')
                })

if failed_tests:
    print('<div class="section fade-in">')
    print('<h2>❌ Failed Tests Summary</h2>')
    for test in failed_tests:
        print(f'<div style="padding: 15px; background: #0f172a; border-left: 4px solid #f87171; margin-bottom: 10px; border-radius: 5px;">')
        print(f'<strong style="color: #f87171;">{html.escape(test["suite"])} / {html.escape(test["command"])} / {html.escape(test["test"])}</strong>')
        if test['notes']:
            print(f'<p style="margin-top: 5px; color: #e2e8f0;">{html.escape(test["notes"])}</p>')
        print('</div>')
    print('</div>')

if rejected_tests:
    print('<div class="section fade-in">')
    print('<h2>🚫 Rejected Commands</h2>')
    for test in rejected_tests:
        print(f'<div style="padding: 15px; background: #0f172a; border-left: 4px solid #f472b6; margin-bottom: 10px; border-radius: 5px;">')
        print(f'<strong style="color: #f472b6;">{html.escape(test["suite"])} / {html.escape(test["command"])} / {html.escape(test["test"])}</strong>')
        if test['notes']:
            print(f'<p style="margin-top: 5px; color: #e2e8f0;">{html.escape(test["notes"])}</p>')
        print('</div>')
    print('</div>')

print(f'<div class="footer">')
print(f'<p>Report Generated: {timestamp}</p>')
print(f'<p>Total Tests: {total_tests} | Coverage: {coverage}%</p>')
print(f'</div>')
PYTHON_SCRIPT

# Close HTML
cat >> "$REPORT_FILE" << 'HTML_END'
        </div>
    </div>
    
    <script>
        // Add fade-in animation to elements
        document.addEventListener('DOMContentLoaded', function() {
            const elements = document.querySelectorAll('.fade-in');
            elements.forEach((el, index) => {
                el.style.opacity = '0';
                setTimeout(() => {
                    el.style.opacity = '1';
                }, index * 100);
            });
        });
    </script>
</body>
</html>
HTML_END

echo -e "${GREEN}✅ HTML Report Generated Successfully!${NC}"
echo ""
echo "📄 Report Location:"
echo "   $REPORT_FILE"
echo ""
echo "Open in browser? (y/n)"
read -p "> " choice

if [ "$choice" = "y" ] || [ "$choice" = "Y" ]; then
    if command -v open &> /dev/null; then
        open "$REPORT_FILE"
    elif command -v xdg-open &> /dev/null; then
        xdg-open "$REPORT_FILE"
    else
        echo "Please open manually: $REPORT_FILE"
    fi
fi