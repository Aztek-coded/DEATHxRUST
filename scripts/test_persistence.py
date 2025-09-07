#!/usr/bin/env python3
"""
Robust persistence layer for command testing system.
Provides automatic backups, data versioning, and recovery capabilities.
"""

import json
import sqlite3
import os
import shutil
from datetime import datetime, timedelta
from pathlib import Path
from typing import Dict, List, Any, Optional
import hashlib
import gzip

class TestPersistence:
    """Handles all persistence operations for the testing system."""
    
    def __init__(self, data_dir: str = "test_results"):
        self.data_dir = Path(data_dir)
        self.data_dir.mkdir(exist_ok=True)
        
        # Database path
        self.db_path = self.data_dir / "test_data.db"
        self.backup_dir = self.data_dir / "backups"
        self.backup_dir.mkdir(exist_ok=True)
        
        # Initialize database
        self._init_database()
        
        # Auto-backup on initialization
        self._auto_backup()
    
    def _init_database(self):
        """Initialize SQLite database with proper schema."""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        # Create tables
        cursor.execute("""
            CREATE TABLE IF NOT EXISTS test_sessions (
                session_id TEXT PRIMARY KEY,
                started_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                ended_at TIMESTAMP,
                user_id TEXT,
                guild_id TEXT,
                channel_id TEXT,
                metadata JSON
            )
        """)
        
        cursor.execute("""
            CREATE TABLE IF NOT EXISTS test_results (
                result_id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id TEXT,
                command TEXT NOT NULL,
                subcommand TEXT,
                status TEXT NOT NULL,
                tested_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                notes TEXT,
                error_output TEXT,
                execution_time_ms INTEGER,
                metadata JSON,
                FOREIGN KEY (session_id) REFERENCES test_sessions(session_id)
            )
        """)
        
        cursor.execute("""
            CREATE TABLE IF NOT EXISTS command_registry (
                command TEXT NOT NULL,
                subcommand TEXT,
                description TEXT,
                aliases JSON,
                discovered_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                last_modified TIMESTAMP,
                is_active BOOLEAN DEFAULT 1,
                PRIMARY KEY (command, subcommand)
            )
        """)
        
        cursor.execute("""
            CREATE TABLE IF NOT EXISTS test_snapshots (
                snapshot_id INTEGER PRIMARY KEY AUTOINCREMENT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                snapshot_data JSON,
                checksum TEXT,
                description TEXT
            )
        """)
        
        # Create indexes for performance
        cursor.execute("CREATE INDEX IF NOT EXISTS idx_test_results_command ON test_results(command, subcommand)")
        cursor.execute("CREATE INDEX IF NOT EXISTS idx_test_results_session ON test_results(session_id)")
        cursor.execute("CREATE INDEX IF NOT EXISTS idx_test_results_status ON test_results(status)")
        
        conn.commit()
        conn.close()
    
    def _auto_backup(self):
        """Automatically backup database if needed."""
        # Check if backup is needed (last backup > 24 hours ago)
        latest_backup = self._get_latest_backup()
        if not latest_backup or (datetime.now() - latest_backup) > timedelta(hours=24):
            self.create_backup("auto")
    
    def _get_latest_backup(self) -> Optional[datetime]:
        """Get timestamp of latest backup."""
        backups = list(self.backup_dir.glob("*.db.gz"))
        if not backups:
            return None
        
        latest = max(backups, key=lambda p: p.stat().st_mtime)
        return datetime.fromtimestamp(latest.stat().st_mtime)
    
    def create_backup(self, backup_type: str = "manual") -> str:
        """Create compressed backup of database."""
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        backup_name = f"test_data_{backup_type}_{timestamp}.db.gz"
        backup_path = self.backup_dir / backup_name
        
        # Compress and save
        with open(self.db_path, 'rb') as f_in:
            with gzip.open(backup_path, 'wb') as f_out:
                shutil.copyfileobj(f_in, f_out)
        
        # Clean old backups (keep last 10)
        self._cleanup_old_backups(keep=10)
        
        return str(backup_path)
    
    def _cleanup_old_backups(self, keep: int = 10):
        """Remove old backups, keeping only the most recent ones."""
        backups = sorted(self.backup_dir.glob("*.db.gz"), key=lambda p: p.stat().st_mtime)
        if len(backups) > keep:
            for backup in backups[:-keep]:
                backup.unlink()
    
    def restore_backup(self, backup_path: str) -> bool:
        """Restore database from backup."""
        backup_file = Path(backup_path)
        if not backup_file.exists():
            return False
        
        # Create safety backup of current state
        self.create_backup("pre_restore")
        
        # Restore from backup
        with gzip.open(backup_file, 'rb') as f_in:
            with open(self.db_path, 'wb') as f_out:
                shutil.copyfileobj(f_in, f_out)
        
        return True
    
    def start_session(self, user_id: str, guild_id: str, channel_id: str) -> str:
        """Start a new test session."""
        session_id = hashlib.sha256(
            f"{user_id}{guild_id}{channel_id}{datetime.now()}".encode()
        ).hexdigest()[:16]
        
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        cursor.execute("""
            INSERT INTO test_sessions (session_id, user_id, guild_id, channel_id)
            VALUES (?, ?, ?, ?)
        """, (session_id, user_id, guild_id, channel_id))
        
        conn.commit()
        conn.close()
        
        return session_id
    
    def end_session(self, session_id: str):
        """End a test session."""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        cursor.execute("""
            UPDATE test_sessions 
            SET ended_at = CURRENT_TIMESTAMP 
            WHERE session_id = ?
        """, (session_id,))
        
        conn.commit()
        conn.close()
    
    def record_test(self, session_id: str, command: str, subcommand: Optional[str],
                   status: str, notes: str = "", error_output: str = "",
                   execution_time_ms: int = 0, metadata: Dict = None):
        """Record a test result."""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        cursor.execute("""
            INSERT INTO test_results 
            (session_id, command, subcommand, status, notes, error_output, execution_time_ms, metadata)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        """, (
            session_id, command, subcommand or "", status, notes, 
            error_output, execution_time_ms, json.dumps(metadata or {})
        ))
        
        conn.commit()
        conn.close()
    
    def update_command_registry(self, commands: Dict[str, Any]):
        """Update the command registry with discovered commands."""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        for cmd_name, cmd_info in commands.items():
            # Update main command
            cursor.execute("""
                INSERT OR REPLACE INTO command_registry 
                (command, subcommand, description, aliases, last_modified)
                VALUES (?, '', ?, ?, CURRENT_TIMESTAMP)
            """, (
                cmd_name, 
                cmd_info.get('description', ''),
                json.dumps(cmd_info.get('aliases', []))
            ))
            
            # Update subcommands
            for subcmd_name, subcmd_info in cmd_info.get('subcommands', {}).items():
                cursor.execute("""
                    INSERT OR REPLACE INTO command_registry 
                    (command, subcommand, description, aliases, last_modified)
                    VALUES (?, ?, ?, ?, CURRENT_TIMESTAMP)
                """, (
                    cmd_name,
                    subcmd_name,
                    subcmd_info.get('description', ''),
                    json.dumps(subcmd_info.get('aliases', []))
                ))
        
        conn.commit()
        conn.close()
    
    def get_test_status(self, command: str, subcommand: Optional[str] = None) -> Dict:
        """Get the latest test status for a command."""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        cursor.execute("""
            SELECT status, tested_at, notes, error_output, execution_time_ms
            FROM test_results
            WHERE command = ? AND subcommand = ?
            ORDER BY tested_at DESC
            LIMIT 1
        """, (command, subcommand or ""))
        
        result = cursor.fetchone()
        conn.close()
        
        if result:
            return {
                'status': result[0],
                'last_tested': result[1],
                'notes': result[2],
                'error_output': result[3],
                'execution_time_ms': result[4]
            }
        return {'status': 'untested'}
    
    def get_all_test_results(self) -> List[Dict]:
        """Get all test results with statistics, integrated with discovered commands."""
        conn = sqlite3.connect(self.db_path)
        conn.row_factory = sqlite3.Row
        cursor = conn.cursor()
        
        cursor.execute("""
            SELECT 
                r.command, r.subcommand, r.status, r.tested_at, r.notes,
                r.error_output, r.execution_time_ms,
                c.description, c.aliases
            FROM test_results r
            LEFT JOIN command_registry c 
                ON r.command = c.command AND r.subcommand = c.subcommand
            WHERE r.tested_at IN (
                SELECT MAX(tested_at)
                FROM test_results
                GROUP BY command, subcommand
            )
            ORDER BY r.command, r.subcommand
        """)
        
        results = []
        for row in cursor.fetchall():
            results.append(dict(row))
        
        conn.close()
        
        # Integrate with discovered commands to include untested commands
        results = self._integrate_with_discovered_commands(results)
        
        return results
    
    def _integrate_with_discovered_commands(self, existing_results: List[Dict]) -> List[Dict]:
        """Integrate test results with discovered commands to include untested commands."""
        discovered_path = "test_results/discovered_commands.json"
        
        if not os.path.exists(discovered_path):
            return existing_results
        
        try:
            with open(discovered_path, 'r') as f:
                discovered_commands = json.load(f)
        except (json.JSONDecodeError, IOError):
            return existing_results
        
        # Create a set of existing command/subcommand pairs
        existing_pairs = set()
        for result in existing_results:
            command = result.get('command', '')
            subcommand = result.get('subcommand', '')
            existing_pairs.add((command, subcommand or None))
        
        # Add missing commands from discovered_commands
        for command, cmd_data in discovered_commands.items():
            # Add base command if it doesn't exist and has description
            if (command, None) not in existing_pairs and cmd_data.get('description'):
                existing_results.append({
                    'command': command,
                    'subcommand': None,
                    'status': 'untested',
                    'tested_at': 'Never',
                    'notes': '',
                    'description': cmd_data['description'],
                    'aliases': cmd_data.get('aliases', []),
                    'error_output': None,
                    'execution_time_ms': None
                })
            
            # Add all subcommands that don't exist
            for sub_name, sub_data in cmd_data.get('subcommands', {}).items():
                if (command, sub_name) not in existing_pairs:
                    existing_results.append({
                        'command': command,
                        'subcommand': sub_name,
                        'status': 'untested',
                        'tested_at': 'Never',
                        'notes': '',
                        'description': sub_data.get('description', ''),
                        'aliases': sub_data.get('aliases', []),
                        'error_output': None,
                        'execution_time_ms': None
                    })
        
        return existing_results
    
    def get_statistics(self) -> Dict:
        """Get comprehensive test statistics."""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        # Overall statistics
        cursor.execute("""
            SELECT 
                COUNT(DISTINCT command || ':' || subcommand) as total_commands,
                COUNT(DISTINCT session_id) as total_sessions,
                COUNT(*) as total_tests,
                AVG(execution_time_ms) as avg_execution_time
            FROM test_results
        """)
        overall = cursor.fetchone()
        
        # Status distribution
        cursor.execute("""
            SELECT status, COUNT(*) as count
            FROM (
                SELECT command, subcommand, status
                FROM test_results
                WHERE tested_at IN (
                    SELECT MAX(tested_at)
                    FROM test_results
                    GROUP BY command, subcommand
                )
            )
            GROUP BY status
        """)
        status_dist = {row[0]: row[1] for row in cursor.fetchall()}
        
        # Recent activity
        cursor.execute("""
            SELECT COUNT(*) as tests_24h
            FROM test_results
            WHERE tested_at > datetime('now', '-1 day')
        """)
        recent = cursor.fetchone()
        
        conn.close()
        
        return {
            'total_commands': overall[0] or 0,
            'total_sessions': overall[1] or 0,
            'total_tests': overall[2] or 0,
            'avg_execution_time_ms': overall[3] or 0,
            'status_distribution': status_dist,
            'tests_last_24h': recent[0] if recent else 0
        }
    
    def create_snapshot(self, description: str = "") -> int:
        """Create a snapshot of current test state."""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        # Get current state
        results = self.get_all_test_results()
        stats = self.get_statistics()
        
        snapshot_data = {
            'timestamp': datetime.now().isoformat(),
            'results': results,
            'statistics': stats
        }
        
        # Calculate checksum
        checksum = hashlib.sha256(
            json.dumps(snapshot_data, sort_keys=True).encode()
        ).hexdigest()
        
        # Store snapshot
        cursor.execute("""
            INSERT INTO test_snapshots (snapshot_data, checksum, description)
            VALUES (?, ?, ?)
        """, (json.dumps(snapshot_data), checksum, description))
        
        snapshot_id = cursor.lastrowid
        conn.commit()
        conn.close()
        
        return snapshot_id
    
    def export_to_json(self, output_path: Optional[str] = None) -> str:
        """Export all data to JSON format."""
        if not output_path:
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            output_path = self.data_dir / f"export_{timestamp}.json"
        
        export_data = {
            'exported_at': datetime.now().isoformat(),
            'results': self.get_all_test_results(),
            'statistics': self.get_statistics(),
            'sessions': self._get_all_sessions()
        }
        
        with open(output_path, 'w') as f:
            json.dump(export_data, f, indent=2, default=str)
        
        return str(output_path)
    
    def _get_all_sessions(self) -> List[Dict]:
        """Get all test sessions."""
        conn = sqlite3.connect(self.db_path)
        conn.row_factory = sqlite3.Row
        cursor = conn.cursor()
        
        cursor.execute("SELECT * FROM test_sessions ORDER BY started_at DESC")
        sessions = [dict(row) for row in cursor.fetchall()]
        
        conn.close()
        return sessions
    
    def migrate_from_json(self, json_path: str) -> bool:
        """Migrate data from existing JSON format to database."""
        try:
            with open(json_path, 'r') as f:
                data = json.load(f)
            
            # Create a migration session
            session_id = self.start_session("migration", "migration", "migration")
            
            # Import each test result
            for command, content in data.items():
                if isinstance(content, dict):
                    for subcommand, info in content.items():
                        if isinstance(info, dict) and 'status' in info:
                            subcmd = None if subcommand == '_self' else subcommand
                            self.record_test(
                                session_id, command, subcmd,
                                info.get('status', 'untested'),
                                info.get('notes', ''),
                                metadata={'migrated': True}
                            )
            
            self.end_session(session_id)
            return True
        except Exception as e:
            print(f"Migration failed: {e}")
            return False


if __name__ == "__main__":
    # Example usage
    persistence = TestPersistence()
    
    # Try to migrate existing data
    if os.path.exists("test_results/universal_test_status.json"):
        print("Migrating existing test data...")
        if persistence.migrate_from_json("test_results/universal_test_status.json"):
            print("Migration successful!")
    
    # Show statistics
    stats = persistence.get_statistics()
    print(f"\nTest Statistics:")
    print(f"  Total Commands: {stats['total_commands']}")
    print(f"  Total Tests: {stats['total_tests']}")
    print(f"  Status Distribution: {stats['status_distribution']}")