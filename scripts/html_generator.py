#!/usr/bin/env python3
"""
Advanced HTML report generator for test results with interactive features.
"""

import json
import os
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Any, Optional
import base64

class HTMLReportGenerator:
    """Generates comprehensive HTML reports with charts and interactivity."""
    
    def __init__(self, persistence=None):
        self.persistence = persistence
        self.template_dir = Path(__file__).parent / "templates"
        
    def generate_report(self, output_path: Optional[str] = None, 
                       title: str = "Command Test Report", 
                       auto_open: bool = False) -> str:
        """Generate comprehensive HTML report."""
        if not output_path:
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            output_path = f"test_results/report_{timestamp}.html"
        
        # Clean up old reports before generating new one
        self._cleanup_old_reports()
        
        # Get data from persistence layer
        if self.persistence:
            results = self.persistence.get_all_test_results()
            stats = self.persistence.get_statistics()
        else:
            # Fallback to JSON if no persistence
            results = self._load_json_results()
            stats = self._calculate_stats(results)
        
        # Generate HTML
        html = self._generate_html(results, stats, title)
        
        # Save report
        with open(output_path, 'w') as f:
            f.write(html)
        
        # Get absolute path for better compatibility
        abs_path = os.path.abspath(output_path)
        
        # Automatically open the report if requested
        if auto_open:
            self._open_report(abs_path)
        
        return abs_path
    
    def _generate_html(self, results: List[Dict], stats: Dict, title: str) -> str:
        """Generate the full HTML report."""
        return f"""<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    {self._get_styles()}
    {self._get_scripts()}
</head>
<body>
    <div class="container">
        {self._generate_header(title)}
        {self._generate_summary(stats)}
        {self._generate_charts(stats)}
        {self._generate_filters()}
        {self._generate_results_table(results)}
        {self._generate_timeline(results)}
        {self._generate_footer()}
    </div>
</body>
</html>"""
    
    def _get_styles(self) -> str:
        """Get CSS styles for the report."""
        return """
    <style>
        :root {
            --bg-primary: #0e0e10;
            --bg-secondary: #18181b;
            --bg-tertiary: #1f1f23;
            --border-color: #3a3a3d;
            --text-primary: #efeff1;
            --text-secondary: #adadb8;
            --accent: #9146ff;
            --accent-hover: #772ce8;
            --success: #00f593;
            --danger: #fb4648;
            --warning: #ffb626;
            --info: #4da5ff;
        }
        
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
            background: var(--bg-primary);
            color: var(--text-primary);
            line-height: 1.6;
        }
        
        .container {
            max-width: 1400px;
            margin: 0 auto;
            padding: 20px;
        }
        
        h1, h2, h3 {
            color: var(--accent);
            margin-bottom: 1rem;
        }
        
        .header {
            text-align: center;
            padding: 2rem 0;
            border-bottom: 2px solid var(--accent);
            margin-bottom: 2rem;
        }
        
        .summary-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 1rem;
            margin: 2rem 0;
        }
        
        .stat-card {
            background: var(--bg-secondary);
            border: 1px solid var(--border-color);
            border-radius: 8px;
            padding: 1.5rem;
            text-align: center;
            transition: transform 0.2s, box-shadow 0.2s;
        }
        
        .stat-card:hover {
            transform: translateY(-2px);
            box-shadow: 0 5px 15px rgba(145, 70, 255, 0.2);
        }
        
        .stat-value {
            font-size: 2.5rem;
            font-weight: bold;
            margin: 0.5rem 0;
        }
        
        .stat-label {
            color: var(--text-secondary);
            font-size: 0.9rem;
            text-transform: uppercase;
            letter-spacing: 1px;
        }
        
        .chart-container {
            background: var(--bg-secondary);
            border: 1px solid var(--border-color);
            border-radius: 8px;
            padding: 1.5rem;
            margin: 2rem 0;
        }
        
        .filters {
            background: var(--bg-secondary);
            border: 1px solid var(--border-color);
            border-radius: 8px;
            padding: 1rem;
            margin: 2rem 0;
            display: flex;
            gap: 1rem;
            flex-wrap: wrap;
            align-items: center;
        }
        
        .filter-group {
            display: flex;
            align-items: center;
            gap: 0.5rem;
        }
        
        .filter-group label {
            color: var(--text-secondary);
            font-size: 0.9rem;
        }
        
        input, select {
            background: var(--bg-tertiary);
            color: var(--text-primary);
            border: 1px solid var(--border-color);
            border-radius: 4px;
            padding: 0.5rem;
            font-size: 0.9rem;
        }
        
        input:focus, select:focus {
            outline: none;
            border-color: var(--accent);
        }
        
        .btn {
            background: var(--accent);
            color: white;
            border: none;
            border-radius: 4px;
            padding: 0.5rem 1rem;
            cursor: pointer;
            font-size: 0.9rem;
            transition: background 0.2s;
        }
        
        .btn:hover {
            background: var(--accent-hover);
        }
        
        /* Collapsible sections styles */
        .collapsible-container {
            background: var(--bg-secondary);
            border-radius: 8px;
            margin: 2rem 0;
            overflow: hidden;
        }
        
        .collapsible-header {
            background: var(--bg-tertiary);
            padding: 1rem;
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: space-between;
            user-select: none;
            transition: background 0.2s;
        }
        
        .collapsible-header:hover {
            background: var(--accent);
            background: rgba(145, 70, 255, 0.2);
        }
        
        .collapsible-header.command-level {
            font-size: 1.1rem;
            font-weight: 600;
            border-bottom: 2px solid var(--border-color);
        }
        
        .collapsible-header.group-level {
            padding-left: 2rem;
            font-size: 1rem;
            background: rgba(145, 70, 255, 0.1);
        }
        
        .collapse-icon {
            font-size: 0.8rem;
            transition: transform 0.3s;
            margin-right: 0.5rem;
        }
        
        .collapsed .collapse-icon {
            transform: rotate(-90deg);
        }
        
        .collapsible-content {
            max-height: 2000px;
            overflow: hidden;
            transition: max-height 0.3s ease;
        }
        
        .collapsed .collapsible-content {
            max-height: 0;
        }
        
        .command-stats {
            display: flex;
            gap: 1rem;
            margin-left: auto;
            align-items: center;
        }
        
        .mini-stat {
            display: flex;
            align-items: center;
            gap: 0.25rem;
            font-size: 0.9rem;
            color: var(--text-secondary);
        }
        
        table {
            width: 100%;
            background: var(--bg-secondary);
            border-collapse: collapse;
        }
        
        thead {
            background: var(--bg-tertiary);
        }
        
        th {
            padding: 1rem;
            text-align: left;
            color: var(--accent);
            font-weight: 600;
            border-bottom: 2px solid var(--border-color);
        }
        
        td {
            padding: 0.75rem 1rem;
            border-bottom: 1px solid var(--border-color);
        }
        
        tr:hover {
            background: var(--bg-tertiary);
        }
        
        .status-badge {
            display: inline-block;
            padding: 0.25rem 0.75rem;
            border-radius: 12px;
            font-size: 0.85rem;
            font-weight: 600;
        }
        
        .status-passed { background: var(--success); color: black; }
        .status-failed { background: var(--danger); color: white; }
        .status-partial { background: var(--warning); color: black; }
        .status-untested { background: var(--text-secondary); color: black; }
        .status-skipped { background: var(--info); color: white; }
        .status-rejected { background: #e91e63; color: white; }
        
        .command-name {
            font-family: 'Courier New', monospace;
            color: var(--info);
        }
        
        .notes {
            color: var(--text-secondary);
            font-size: 0.85rem;
            font-style: italic;
        }
        
        .timestamp {
            color: var(--text-secondary);
            font-size: 0.85rem;
        }
        
        .timeline {
            background: var(--bg-secondary);
            border: 1px solid var(--border-color);
            border-radius: 8px;
            padding: 1.5rem;
            margin: 2rem 0;
        }
        
        .timeline-item {
            display: flex;
            align-items: center;
            padding: 0.5rem 0;
            border-left: 2px solid var(--border-color);
            margin-left: 1rem;
            padding-left: 1rem;
            position: relative;
        }
        
        .timeline-item::before {
            content: '';
            position: absolute;
            left: -6px;
            width: 10px;
            height: 10px;
            border-radius: 50%;
            background: var(--accent);
        }
        
        .progress-bar {
            width: 100%;
            height: 30px;
            background: var(--bg-tertiary);
            border-radius: 15px;
            overflow: hidden;
            margin: 1rem 0;
        }
        
        .progress-fill {
            height: 100%;
            background: linear-gradient(90deg, var(--accent), var(--accent-hover));
            transition: width 0.5s ease;
        }
        
        .footer {
            text-align: center;
            padding: 2rem 0;
            margin-top: 3rem;
            border-top: 1px solid var(--border-color);
            color: var(--text-secondary);
        }
        
        .expand-controls {
            margin: 1rem 0;
            display: flex;
            gap: 1rem;
        }
        
        .expand-btn {
            background: var(--bg-tertiary);
            border: 1px solid var(--border-color);
            color: var(--text-secondary);
            padding: 0.5rem 1rem;
            border-radius: 4px;
            cursor: pointer;
            transition: all 0.2s;
        }
        
        .expand-btn:hover {
            background: var(--accent);
            color: white;
            border-color: var(--accent);
        }
        
        @media (max-width: 768px) {
            .summary-grid {
                grid-template-columns: 1fr;
            }
            
            .filters {
                flex-direction: column;
                align-items: stretch;
            }
            
            table {
                font-size: 0.85rem;
            }
            
            th, td {
                padding: 0.5rem;
            }
            
            .collapsible-header.group-level {
                padding-left: 1rem;
            }
        }
    </style>"""
    
    def _get_scripts(self) -> str:
        """Get JavaScript for interactivity."""
        return """
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <script>
        // Collapsible functionality
        function toggleCollapse(element) {
            const parent = element.closest('.collapsible-section');
            parent.classList.toggle('collapsed');
        }
        
        function expandAll() {
            document.querySelectorAll('.collapsible-section').forEach(section => {
                section.classList.remove('collapsed');
            });
        }
        
        function collapseAll() {
            document.querySelectorAll('.collapsible-section').forEach(section => {
                section.classList.add('collapsed');
            });
        }
        
        // Filter functionality for collapsible structure
        function filterResults() {
            const searchInput = document.getElementById('searchInput').value.toLowerCase();
            const statusFilter = document.getElementById('statusFilter').value;
            
            // Filter individual rows in tables
            document.querySelectorAll('.results-table tbody tr').forEach(row => {
                const command = row.cells[0].textContent.toLowerCase();
                const status = row.querySelector('.status-badge')?.textContent.toLowerCase() || '';
                
                const matchesSearch = command.includes(searchInput);
                const matchesStatus = statusFilter === 'all' || status.includes(statusFilter);
                
                row.style.display = matchesSearch && matchesStatus ? '' : 'none';
            });
            
            // Hide empty sections
            document.querySelectorAll('.collapsible-section').forEach(section => {
                const visibleRows = section.querySelectorAll('tbody tr:not([style*="display: none"])');
                section.style.display = visibleRows.length > 0 ? '' : 'none';
            });
            
            updateVisibleCount();
        }
        
        function updateVisibleCount() {
            const visibleRows = document.querySelectorAll('.results-table tbody tr:not([style*="display: none"])');
            const countElement = document.getElementById('visibleCount');
            if (countElement) {
                countElement.textContent = visibleRows.length;
            }
        }
        
        // Sort table within a section
        function sortTable(tableElement, column) {
            const tbody = tableElement.querySelector('tbody');
            const rows = Array.from(tbody.querySelectorAll('tr'));
            
            const sortedRows = rows.sort((a, b) => {
                const aValue = a.cells[column].textContent;
                const bValue = b.cells[column].textContent;
                return aValue.localeCompare(bValue);
            });
            
            tbody.innerHTML = '';
            sortedRows.forEach(row => tbody.appendChild(row));
        }
        
        // Export functionality
        function exportToCSV() {
            const rows = document.querySelectorAll('.results-table tr:not([style*="display: none"])');
            let csv = [];
            
            // Add headers
            csv.push(['Command', 'Subcommand', 'Description', 'Status', 'Last Tested', 'Notes'].join(','));
            
            rows.forEach(row => {
                if (row.querySelector('th')) return; // Skip header rows
                const cells = row.querySelectorAll('td');
                if (cells.length > 0) {
                    const rowData = Array.from(cells).map(cell => 
                        '"' + cell.textContent.trim().replace(/"/g, '""') + '"'
                    );
                    csv.push(rowData.join(','));
                }
            });
            
            const blob = new Blob([csv.join('\\n')], { type: 'text/csv' });
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = 'test_results_' + new Date().toISOString().slice(0, 10) + '.csv';
            a.click();
        }
        
        // Initialize on load
        document.addEventListener('DOMContentLoaded', function() {
            // Initialize charts
            initCharts();
            
            // Add event listeners
            document.getElementById('searchInput')?.addEventListener('input', filterResults);
            document.getElementById('statusFilter')?.addEventListener('change', filterResults);
            document.getElementById('exportBtn')?.addEventListener('click', exportToCSV);
            document.getElementById('expandAllBtn')?.addEventListener('click', expandAll);
            document.getElementById('collapseAllBtn')?.addEventListener('click', collapseAll);
            
            // Add click handlers to all collapsible headers
            document.querySelectorAll('.collapsible-header').forEach(header => {
                header.addEventListener('click', function() {
                    toggleCollapse(this);
                });
            });
            
            // Initial count
            updateVisibleCount();
        });
        
        function initCharts() {
            // Status distribution pie chart
            const statusCtx = document.getElementById('statusChart')?.getContext('2d');
            if (statusCtx && window.statusData) {
                new Chart(statusCtx, {
                    type: 'doughnut',
                    data: {
                        labels: Object.keys(window.statusData),
                        datasets: [{
                            data: Object.values(window.statusData),
                            backgroundColor: [
                                '#00f593',
                                '#fb4648',
                                '#ffb626',
                                '#adadb8',
                                '#4da5ff',
                                '#e91e63'
                            ]
                        }]
                    },
                    options: {
                        responsive: true,
                        maintainAspectRatio: false,
                        plugins: {
                            legend: {
                                position: 'right',
                                labels: { color: '#efeff1' }
                            }
                        }
                    }
                });
            }
            
            // Timeline chart
            const timelineCtx = document.getElementById('timelineChart')?.getContext('2d');
            if (timelineCtx && window.timelineData) {
                new Chart(timelineCtx, {
                    type: 'line',
                    data: window.timelineData,
                    options: {
                        responsive: true,
                        maintainAspectRatio: false,
                        scales: {
                            x: {
                                ticks: { color: '#efeff1' },
                                grid: { color: '#3a3a3d' }
                            },
                            y: {
                                ticks: { color: '#efeff1' },
                                grid: { color: '#3a3a3d' }
                            }
                        },
                        plugins: {
                            legend: {
                                labels: { color: '#efeff1' }
                            }
                        }
                    }
                });
            }
        }
    </script>"""
    
    def _generate_header(self, title: str) -> str:
        """Generate report header."""
        return f"""
        <div class="header">
            <h1>🚀 {title}</h1>
            <p class="timestamp">Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}</p>
        </div>"""
    
    def _generate_summary(self, stats: Dict) -> str:
        """Generate summary statistics cards."""
        total = sum(stats.get('status_distribution', {}).values())
        passed = stats.get('status_distribution', {}).get('passed', 0)
        coverage = (passed * 100 // total) if total > 0 else 0
        
        return f"""
        <div class="summary-grid">
            <div class="stat-card">
                <div class="stat-label">Total Commands</div>
                <div class="stat-value">{stats.get('total_commands', 0)}</div>
            </div>
            <div class="stat-card">
                <div class="stat-label">Test Coverage</div>
                <div class="stat-value">{coverage}%</div>
                <div class="progress-bar">
                    <div class="progress-fill" style="width: {coverage}%"></div>
                </div>
            </div>
            <div class="stat-card">
                <div class="stat-label">Total Tests</div>
                <div class="stat-value">{stats.get('total_tests', 0)}</div>
            </div>
            <div class="stat-card">
                <div class="stat-label">Tests (24h)</div>
                <div class="stat-value">{stats.get('tests_last_24h', 0)}</div>
            </div>
        </div>"""
    
    def _generate_charts(self, stats: Dict) -> str:
        """Generate chart containers with data."""
        status_dist = stats.get('status_distribution', {})
        
        # Prepare data for JavaScript
        status_data = json.dumps(status_dist)
        
        return f"""
        <div class="chart-container">
            <h2>📊 Status Distribution</h2>
            <div style="height: 300px;">
                <canvas id="statusChart"></canvas>
            </div>
        </div>
        <script>
            window.statusData = {status_data};
        </script>"""
    
    def _generate_filters(self) -> str:
        """Generate filter controls."""
        return """
        <div class="filters">
            <div class="filter-group">
                <label for="searchInput">Search:</label>
                <input type="text" id="searchInput" placeholder="Filter commands...">
            </div>
            <div class="filter-group">
                <label for="statusFilter">Status:</label>
                <select id="statusFilter">
                    <option value="all">All</option>
                    <option value="passed">Passed</option>
                    <option value="failed">Failed</option>
                    <option value="partial">Partial</option>
                    <option value="untested">Untested</option>
                    <option value="skipped">Skipped</option>
                    <option value="rejected">Rejected</option>
                </select>
            </div>
            <div class="filter-group">
                <span>Showing: <strong id="visibleCount">0</strong> results</span>
            </div>
            <button class="btn" id="exportBtn">📥 Export CSV</button>
        </div>
        <div class="expand-controls">
            <button class="expand-btn" id="expandAllBtn">➕ Expand All</button>
            <button class="expand-btn" id="collapseAllBtn">➖ Collapse All</button>
        </div>"""
    
    def _generate_results_table(self, results: List[Dict]) -> str:
        """Generate the main results table with collapsible hierarchy."""
        from collections import defaultdict
        
        # Organize results by command and subcommand groups
        command_structure = defaultdict(lambda: {'_self': None, 'groups': defaultdict(lambda: {'_self': None, 'items': []}), 'direct_subs': []})
        
        for result in results:
            command = result.get('command', '')
            subcommand = result.get('subcommand', '')
            
            if not subcommand or subcommand == '_self':
                # Base command
                command_structure[command]['_self'] = result
            else:
                # Check if subcommand has a space (indicates a group)
                parts = subcommand.split(' ', 1)
                if len(parts) > 1:
                    # Subcommand group with nested item
                    group_name = parts[0]
                    sub_name = parts[1]
                    result['sub_name'] = sub_name
                    result['full_subcommand'] = subcommand
                    command_structure[command]['groups'][group_name]['items'].append(result)
                else:
                    # Could be either a direct subcommand or a group parent
                    # Check if this appears as a group name in any other results
                    is_group_parent = any(
                        r.get('subcommand', '') and r.get('subcommand', '').startswith(f"{subcommand} ")
                        for r in results
                        if r.get('command') == command
                    )
                    
                    if is_group_parent:
                        # This is a group parent command (like "filter" or "award")
                        command_structure[command]['groups'][subcommand]['_self'] = result
                    else:
                        # Direct subcommand
                        command_structure[command]['direct_subs'].append(result)
        
        html_sections = ['<h2>📋 Test Results</h2>']
        
        # Generate HTML for each command
        for cmd_name in sorted(command_structure.keys()):
            cmd_data = command_structure[cmd_name]
            
            # Calculate stats for this command
            all_results = []
            if cmd_data['_self']:
                all_results.append(cmd_data['_self'])
            all_results.extend(cmd_data['direct_subs'])
            for group_data in cmd_data['groups'].values():
                if group_data['_self']:
                    all_results.append(group_data['_self'])
                all_results.extend(group_data['items'])
            
            total = len(all_results)
            passed = sum(1 for r in all_results if r.get('status') == 'passed')
            failed = sum(1 for r in all_results if r.get('status') == 'failed')
            
            # Command-level collapsible section
            html_sections.append(f'''
            <div class="collapsible-section collapsible-container">
                <div class="collapsible-header command-level">
                    <div style="display: flex; align-items: center;">
                        <span class="collapse-icon">▼</span>
                        <span class="command-name">/{cmd_name}</span>
                    </div>
                    <div class="command-stats">
                        <span class="mini-stat">Total: {total}</span>
                        <span class="mini-stat" style="color: var(--success);">✅ {passed}</span>
                        <span class="mini-stat" style="color: var(--danger);">❌ {failed}</span>
                    </div>
                </div>
                <div class="collapsible-content">
            ''')
            
            # Start table for this command
            html_sections.append('''
                <table class="results-table">
                    <thead>
                        <tr>
                            <th>Subcommand</th>
                            <th>Description</th>
                            <th>Status</th>
                            <th>Last Tested</th>
                            <th>Notes</th>
                        </tr>
                    </thead>
                    <tbody>
            ''')
            
            # Add base command if exists
            if cmd_data['_self']:
                html_sections.append(self._generate_row(cmd_data['_self'], is_base=True))
            
            # Add direct subcommands
            for sub_result in sorted(cmd_data['direct_subs'], key=lambda x: x.get('subcommand', '')):
                html_sections.append(self._generate_row(sub_result))
            
            # Add grouped subcommands
            for group_name in sorted(cmd_data['groups'].keys()):
                group_data = cmd_data['groups'][group_name]
                if group_data['_self'] or group_data['items']:
                    # Calculate group stats
                    group_items = group_data['items']
                    if group_data['_self']:
                        group_items = [group_data['_self']] + group_items
                    
                    group_total = len(group_items)
                    group_passed = sum(1 for r in group_items if r.get('status') == 'passed')
                    group_failed = sum(1 for r in group_items if r.get('status') == 'failed')
                    
                    # Add collapsible group section
                    html_sections.append(f'''
                    </tbody>
                    </table>
                    
                    <div class="collapsible-section" style="margin-left: 2rem; margin-top: 0.5rem;">
                        <div class="collapsible-header group-level" onclick="toggleCollapse(this)">
                            <div style="display: flex; align-items: center;">
                                <span class="collapse-icon">▼</span>
                                <span class="command-name" style="color: var(--accent);">/{cmd_name} {group_name}</span>
                            </div>
                            <div class="command-stats">
                                <span class="mini-stat">Total: {group_total}</span>
                                <span class="mini-stat" style="color: var(--success);">✅ {group_passed}</span>
                                <span class="mini-stat" style="color: var(--danger);">❌ {group_failed}</span>
                            </div>
                        </div>
                        <div class="collapsible-content">
                            <table class="results-table">
                                <tbody>
                    ''')
                    
                    # Add the group parent command if it exists
                    if group_data['_self']:
                        html_sections.append(self._generate_row(group_data['_self'], is_group_parent=True))
                    
                    # Add subcommands in this group
                    for sub_result in sorted(group_data['items'], key=lambda x: x.get('sub_name', '')):
                        html_sections.append(self._generate_row(sub_result, indent=True))
                    
                    html_sections.append('''
                                </tbody>
                            </table>
                        </div>
                    </div>
                    
                    <table class="results-table">
                        <tbody>
                    ''')
            
            # Close table and section
            html_sections.append('''
                    </tbody>
                </table>
                </div>
            </div>
            ''')
        
        return '\n'.join(html_sections)
    
    def _generate_row(self, result: Dict, is_base: bool = False, is_group_parent: bool = False, indent: bool = False) -> str:
        """Generate a single table row for a test result."""
        command = result.get('command', '')
        subcommand = result.get('subcommand', '')
        
        # Format the display name
        if is_base:
            display_name = '(base command)'
        elif is_group_parent:
            display_name = f'{subcommand} (group parent)'
        elif 'sub_name' in result:
            display_name = result['sub_name']
        else:
            display_name = subcommand or '(base)'
        
        status = result.get('status', 'untested')
        tested_at = result.get('tested_at', 'Never')
        if tested_at != 'Never':
            try:
                dt = datetime.fromisoformat(tested_at.replace('Z', '+00:00'))
                tested_at = dt.strftime('%Y-%m-%d %H:%M')
            except:
                pass
        
        notes = result.get('notes', '')
        description = result.get('description', '')
        
        # Apply indentation if needed
        padding = 'padding-left: 3rem;' if indent else 'padding-left: 1.5rem;' if not is_base else ''
        
        # Add fallback for empty descriptions
        desc_display = description[:60] if description.strip() else "No description available"
        
        return f'''
        <tr>
            <td class="command-name" style="{padding}">{display_name}</td>
            <td>{desc_display}</td>
            <td><span class="status-badge status-{status}">{status}</span></td>
            <td class="timestamp">{tested_at}</td>
            <td class="notes">{notes[:100]}</td>
        </tr>'''
    
    def _generate_timeline(self, results: List[Dict]) -> str:
        """Generate activity timeline."""
        # Get recent tests
        recent = sorted(
            [r for r in results if r.get('tested_at') and r.get('tested_at') != 'Never'],
            key=lambda x: x.get('tested_at', ''),
            reverse=True
        )[:10]
        
        if not recent:
            return ""
        
        items = []
        for result in recent:
            command = result.get('command', '')
            subcommand = result.get('subcommand', '')
            full_command = f"/{command}"
            if subcommand:
                full_command += f" {subcommand}"
            
            status = result.get('status', 'untested')
            tested_at = result.get('tested_at', '')
            
            items.append(f"""
            <div class="timeline-item">
                <span class="command-name">{full_command}</span>
                <span style="margin: 0 1rem;">→</span>
                <span class="status-badge status-{status}">{status}</span>
                <span style="margin-left: auto;" class="timestamp">{tested_at}</span>
            </div>""")
        
        return f"""
        <div class="timeline">
            <h2>📅 Recent Activity</h2>
            {''.join(items)}
        </div>"""
    
    def _generate_footer(self) -> str:
        """Generate report footer."""
        return """
        <div class="footer">
            <p>Generated by Universal Command Testing System</p>
            <p>© 2024 - Powered by Python & SQLite</p>
        </div>"""
    
    def _load_json_results(self) -> List[Dict]:
        """Load results from JSON file (fallback) and integrate with discovered commands."""
        json_path = "test_results/universal_test_status.json"
        discovered_path = "test_results/discovered_commands.json"
        
        # Load existing test results
        test_data = {}
        if os.path.exists(json_path):
            with open(json_path, 'r') as f:
                test_data = json.load(f)
        
        # Load discovered commands for structure and descriptions
        discovered_commands = {}
        if os.path.exists(discovered_path):
            with open(discovered_path, 'r') as f:
                discovered_commands = json.load(f)
        
        results = []
        
        # Process test results first
        for command, content in test_data.items():
            if isinstance(content, dict):
                for subcommand, info in content.items():
                    if isinstance(info, dict) and 'status' in info:
                        # Get description from discovered commands if available
                        description = ''
                        if command in discovered_commands:
                            cmd_data = discovered_commands[command]
                            if subcommand == '_self' and 'description' in cmd_data:
                                description = cmd_data['description']
                            elif 'subcommands' in cmd_data and subcommand in cmd_data['subcommands']:
                                description = cmd_data['subcommands'][subcommand].get('description', '')
                        
                        results.append({
                            'command': command,
                            'subcommand': None if subcommand == '_self' else subcommand,
                            'status': info.get('status', 'untested'),
                            'tested_at': info.get('last_tested', 'Never'),
                            'notes': info.get('notes', ''),
                            'description': description
                        })
        
        # Add missing commands from discovered_commands that aren't in test results yet
        for command, cmd_data in discovered_commands.items():
            if command not in test_data:
                # Add base command if it has description
                if cmd_data.get('description'):
                    results.append({
                        'command': command,
                        'subcommand': None,
                        'status': 'untested',
                        'tested_at': 'Never',
                        'notes': '',
                        'description': cmd_data['description']
                    })
                
                # Add all subcommands
                for sub_name, sub_data in cmd_data.get('subcommands', {}).items():
                    results.append({
                        'command': command,
                        'subcommand': sub_name,
                        'status': 'untested',
                        'tested_at': 'Never', 
                        'notes': '',
                        'description': sub_data.get('description', '')
                    })
            else:
                # Command exists in test data, add missing subcommands from discovered data
                existing_subs = set(test_data[command].keys())
                for sub_name, sub_data in cmd_data.get('subcommands', {}).items():
                    if sub_name not in existing_subs:
                        results.append({
                            'command': command,
                            'subcommand': sub_name,
                            'status': 'untested',
                            'tested_at': 'Never',
                            'notes': '',
                            'description': sub_data.get('description', '')
                        })
        
        return results
    
    def _calculate_stats(self, results: List[Dict]) -> Dict:
        """Calculate statistics from results."""
        status_dist = {}
        for result in results:
            status = result.get('status', 'untested')
            status_dist[status] = status_dist.get(status, 0) + 1
        
        return {
            'total_commands': len(results),
            'total_tests': len(results),
            'status_distribution': status_dist,
            'tests_last_24h': 0
        }
    
    def _cleanup_old_reports(self, max_reports: int = 5):
        """Clean up old HTML reports, keeping only the most recent ones."""
        import glob
        
        # Find all report files
        report_pattern = "test_results/report_*.html"
        report_files = glob.glob(report_pattern)
        
        if len(report_files) <= max_reports:
            return  # Nothing to clean up
        
        # Sort by modification time (newest first)
        report_files.sort(key=lambda x: os.path.getmtime(x), reverse=True)
        
        # Keep only the most recent max_reports files
        files_to_keep = report_files[:max_reports]
        files_to_delete = report_files[max_reports:]
        
        # Delete old files
        deleted_count = 0
        for file_path in files_to_delete:
            try:
                os.remove(file_path)
                deleted_count += 1
            except OSError:
                pass  # Ignore errors (file might be in use)
        
        if deleted_count > 0:
            print(f"🧹 Cleaned up {deleted_count} old HTML reports (keeping {len(files_to_keep)} most recent)")
    
    def _open_report(self, report_path: str):
        """Open the HTML report in the default browser."""
        import subprocess
        import platform
        
        try:
            system = platform.system().lower()
            if system == "darwin":  # macOS
                subprocess.run(["open", report_path], check=False)
            elif system == "linux":
                subprocess.run(["xdg-open", report_path], check=False)
            elif system == "windows":
                subprocess.run(["start", "", report_path], shell=True, check=False)
            else:
                print(f"📄 Report saved: {report_path}")
                print("Please open manually in your browser")
        except Exception:
            print(f"📄 Report saved: {report_path}")
            print("Please open manually in your browser")


if __name__ == "__main__":
    # Example usage
    from test_persistence import TestPersistence
    
    persistence = TestPersistence()
    generator = HTMLReportGenerator(persistence)
    
    report_path = generator.generate_report(
        title="Discord Bot Command Test Report"
    )
    
    print(f"Report generated: {report_path}")