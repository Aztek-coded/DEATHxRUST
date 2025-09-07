#!/usr/bin/env python3
"""
Command extractor for discovering all available bot commands from Rust source.
"""

import re
import json
import os
from pathlib import Path
from typing import Dict, List, Any

class CommandExtractor:
    def __init__(self, src_path: str = "src"):
        self.src_path = Path(src_path)
        self.commands = {}
        
    def extract_commands(self) -> Dict[str, Any]:
        """Extract all commands from the source code."""
        # First get top-level commands from framework.rs
        framework_file = self.src_path / "bot" / "framework.rs"
        if framework_file.exists():
            self._parse_framework(framework_file)
        
        # Then parse individual command files for details
        commands_dir = self.src_path / "commands"
        if commands_dir.exists():
            self._parse_command_files(commands_dir)
            
        return self.commands
    
    def _parse_framework(self, file_path: Path):
        """Parse framework.rs to get registered commands."""
        with open(file_path, 'r') as f:
            content = f.read()
            
        # Find the commands vector
        commands_pattern = r'let mut commands = vec!\[(.*?)\];'
        match = re.search(commands_pattern, content, re.DOTALL)
        
        if match:
            commands_text = match.group(1)
            # Extract command registrations like ping::ping()
            cmd_pattern = r'(\w+)::(\w+)\(\)'
            for match in re.finditer(cmd_pattern, commands_text):
                module, func = match.groups()
                self.commands[module] = {
                    'name': module,
                    'function': func,
                    'subcommands': {},
                    'description': '',
                    'aliases': []
                }
    
    def _parse_command_files(self, commands_dir: Path):
        """Parse individual command files for details."""
        for item in commands_dir.iterdir():
            if item.is_dir():
                # Handle command modules with subcommands
                self._parse_command_module(item)
            elif item.suffix == '.rs' and item.stem != 'mod':
                # Handle single command files
                self._parse_single_command(item)
    
    def _parse_command_module(self, module_dir: Path):
        """Parse a command module directory (e.g., boosterrole/)."""
        module_name = module_dir.name
        
        # Parse mod.rs for the main command
        mod_file = module_dir / "mod.rs"
        if mod_file.exists():
            with open(mod_file, 'r') as f:
                content = f.read()
                
            # Find main command definition
            cmd_pattern = r'#\[poise::command\((.*?)\)\].*?pub async fn (\w+)'
            match = re.search(cmd_pattern, content, re.DOTALL)
            
            if match:
                attrs, func_name = match.groups()
                
                # Parse attributes
                desc_match = re.search(r'description_localized.*?"en-US".*?"([^"]+)"', attrs)
                if not desc_match:
                    desc_match = re.search(r'description\s*=\s*"([^"]+)"', attrs)
                
                aliases_match = re.search(r'aliases\((.*?)\)', attrs)
                aliases = []
                if aliases_match:
                    aliases = [a.strip().strip('"') for a in aliases_match.group(1).split(',')]
                
                # Check prefix/slash support
                supports_prefix = 'prefix_command' in attrs
                supports_slash = 'slash_command' in attrs
                
                if module_name in self.commands:
                    self.commands[module_name]['description'] = desc_match.group(1) if desc_match else ''
                    self.commands[module_name]['aliases'] = aliases
                    self.commands[module_name]['supports_prefix'] = supports_prefix
                    self.commands[module_name]['supports_slash'] = supports_slash
                
                # Find subcommands
                subcommand_pattern = r'subcommands\((.*?)\)'
                subcommand_match = re.search(subcommand_pattern, attrs, re.DOTALL)
                if subcommand_match:
                    subcmds_text = subcommand_match.group(1)
                    # Clean up and split by comma, handling multi-line
                    subcmds_text = re.sub(r'\s+', ' ', subcmds_text)
                    subcmd_names = [s.strip().strip('"') for s in subcmds_text.split(',') if s.strip()]
                    
                    # Parse each subcommand file
                    for subcmd in subcmd_names:
                        subcmd_file = module_dir / f"{subcmd}.rs"
                        if subcmd_file.exists():
                            self._parse_subcommand(module_name, subcmd, subcmd_file)
    
    def _parse_subcommand(self, parent_cmd: str, subcmd_name: str, file_path: Path):
        """Parse a subcommand file."""
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Find all command definitions in the file
        cmd_pattern = r'#\[poise::command\((.*?)\)\].*?(?:pub )?async fn (\w+)'
        matches = list(re.finditer(cmd_pattern, content, re.DOTALL))
        
        # Look for the main subcommand function
        main_func = None
        sub_subcommands = []
        
        for match in matches:
            attrs, func_name = match.groups()
            
            # Check if this has nested subcommands
            if 'subcommands(' in attrs:
                main_func = func_name
                # Extract nested subcommand names
                subcmd_match = re.search(r'subcommands\((.*?)\)', attrs, re.DOTALL)
                if subcmd_match:
                    subcmds_text = subcmd_match.group(1)
                    subcmds_text = re.sub(r'\s+', ' ', subcmds_text)
                    sub_subcommands = [s.strip().strip('"') for s in subcmds_text.split(',') if s.strip()]
        
        # If there are nested subcommands, use those instead
        if sub_subcommands:
            # This is a parent subcommand with its own subcommands
            # First, add the parent subcommand itself
            parent_desc = ""
            parent_aliases = []
            
            # Find the main function that defines the parent subcommand
            for match in matches:
                attrs, func_name = match.groups()
                # Match either the main_func or the subcmd_name itself (for cases where they're the same)
                if func_name == main_func or func_name == subcmd_name:
                    parent_desc = self._extract_description(attrs, content, match.start())
                    aliases_match = re.search(r'aliases\((.*?)\)', attrs)
                    if aliases_match:
                        aliases_text = aliases_match.group(1)
                        parent_aliases = [a.strip().strip('"') for a in aliases_text.split(',') if a.strip()]
                    break
            
            # Add parent subcommand entry
            if parent_cmd in self.commands:
                self.commands[parent_cmd]['subcommands'][subcmd_name] = {
                    'name': subcmd_name,
                    'description': parent_desc,
                    'aliases': parent_aliases
                }
            
            # Then add the nested subcommands
            for sub_subcmd in sub_subcommands:
                # Find the function definition for this sub-subcommand
                for match in matches:
                    attrs, func_name = match.groups()
                    if func_name == sub_subcmd:
                        # Parse description from parameter attributes or command attributes
                        desc = self._extract_description(attrs, content, match.start())
                        
                        # Parse aliases
                        aliases_match = re.search(r'aliases\((.*?)\)', attrs)
                        aliases = []
                        if aliases_match:
                            aliases_text = aliases_match.group(1)
                            aliases = [a.strip().strip('"') for a in aliases_text.split(',') if a.strip()]
                        
                        # Store as nested subcommand
                        nested_name = f"{subcmd_name} {sub_subcmd}"
                        if parent_cmd in self.commands:
                            self.commands[parent_cmd]['subcommands'][nested_name] = {
                                'name': nested_name,
                                'description': desc,
                                'aliases': aliases,
                                'parent_subcommand': subcmd_name
                            }
        else:
            # Regular subcommand without nesting
            for match in matches:
                attrs, func_name = match.groups()
                if func_name == subcmd_name:
                    desc = self._extract_description(attrs, content, match.start())
                    
                    # Parse aliases
                    aliases_match = re.search(r'aliases\((.*?)\)', attrs)
                    aliases = []
                    if aliases_match:
                        aliases_text = aliases_match.group(1)
                        aliases = [a.strip().strip('"') for a in aliases_text.split(',') if a.strip()]
                    
                    # Add to parent command
                    if parent_cmd in self.commands:
                        self.commands[parent_cmd]['subcommands'][subcmd_name] = {
                            'name': subcmd_name,
                            'description': desc,
                            'aliases': aliases
                        }
                    break
    
    def _extract_description(self, attrs: str, content: str, position: int) -> str:
        """Extract description from various sources."""
        # Try description_localized first (handle multiline)
        # Look for the pattern: description_localized(\n        "en-US",\n        "DESCRIPTION"\n    )
        desc_match = re.search(r'description_localized\s*\(\s*"en-US"\s*,\s*"([^"]+)"\s*\)', attrs, re.DOTALL)
        if desc_match:
            return desc_match.group(1)
        
        # Try regular description attribute
        desc_match = re.search(r'description\s*=\s*"([^"]+)"', attrs)
        if desc_match:
            return desc_match.group(1)
        
        # Look for #[description = "..."] in function parameters
        func_end = content.find('{', position)
        if func_end > 0:
            func_section = content[position:func_end]
            param_desc = re.search(r'#\[description\s*=\s*"([^"]+)"\]', func_section)
            if param_desc:
                return param_desc.group(1)
        
        return ""
    
    def _parse_single_command(self, file_path: Path):
        """Parse a single command file."""
        cmd_name = file_path.stem
        
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Find command definition
        cmd_pattern = r'#\[poise::command\((.*?)\)\].*?pub async fn (\w+)'
        match = re.search(cmd_pattern, content, re.DOTALL)
        
        if match:
            attrs, func_name = match.groups()
            
            # Parse description
            desc_match = re.search(r'description_localized.*?"en-US".*?"([^"]+)"', attrs)
            if not desc_match:
                desc_match = re.search(r'description\s*=\s*"([^"]+)"', attrs)
            
            # Parse subcommands
            subcommands = []
            subcommand_match = re.search(r'subcommands\s*\(\s*([^)]+)\s*\)', attrs)
            if subcommand_match:
                subcmds_text = subcommand_match.group(1)
                subcommands = [s.strip().strip('"').strip("'") for s in subcmds_text.split(',')]
                
                # Parse each subcommand function in the same file
                for subcmd_name in subcommands:
                    subcmd_pattern = rf'#\[poise::command\((.*?)\)\].*?pub async fn {re.escape(subcmd_name)}\('
                    subcmd_match = re.search(subcmd_pattern, content, re.DOTALL)
                    if subcmd_match:
                        subcmd_attrs = subcmd_match.group(1)
                        
                        # Parse subcommand description
                        subcmd_desc_match = re.search(r'description_localized.*?"en-US".*?"([^"]+)"', subcmd_attrs)
                        if not subcmd_desc_match:
                            subcmd_desc_match = re.search(r'description\s*=\s*"([^"]+)"', subcmd_attrs)
                        
                        subcmd_description = subcmd_desc_match.group(1) if subcmd_desc_match else ''
                        
                        # Parse subcommand parameters (simplified)
                        subcmd_params = []
            
            # Parse aliases
            aliases = []
            aliases_match = re.search(r'aliases\s*\(\s*([^)]+)\s*\)', attrs)
            if aliases_match:
                aliases_text = aliases_match.group(1)
                aliases = [a.strip().strip('"').strip("'") for a in aliases_text.split(',')]
            
            # Check if command supports prefix commands
            supports_prefix = 'prefix_command' in attrs
            supports_slash = 'slash_command' in attrs
            
            # Update or create command info
            if cmd_name in self.commands:
                self.commands[cmd_name]['description'] = desc_match.group(1) if desc_match else ''
                if subcommands:
                    self.commands[cmd_name]['subcommands'] = {}
                    for subcmd_name in subcommands:
                        # Look for function with rename = "subcmd_name" or function name matching subcmd_name
                        subcmd_pattern = rf'#\[poise::command\((.*?)\)\].*?pub async fn (\w+)\('
                        subcmd_matches = list(re.finditer(subcmd_pattern, content, re.DOTALL))
                        
                        subcmd_found = False
                        for subcmd_match in subcmd_matches:
                            subcmd_attrs, func_name = subcmd_match.groups()
                            
                            # Check if this function is renamed to our subcmd_name
                            rename_match = re.search(rf'rename\s*=\s*"{re.escape(subcmd_name)}"', subcmd_attrs)
                            if rename_match or func_name == subcmd_name:
                                # Parse subcommand description
                                subcmd_desc_match = re.search(r'description_localized.*?"en-US".*?"([^"]+)"', subcmd_attrs)
                                if not subcmd_desc_match:
                                    subcmd_desc_match = re.search(r'description\s*=\s*"([^"]+)"', subcmd_attrs)
                                
                                subcmd_description = subcmd_desc_match.group(1) if subcmd_desc_match else ''
                                
                                # Parse subcommand parameters (simplified)
                                subcmd_params = []
                                
                                self.commands[cmd_name]['subcommands'][subcmd_name] = {
                                    'description': subcmd_description,
                                    'parameters': subcmd_params
                                }
                                subcmd_found = True
                                break
                        
                        if not subcmd_found:
                            # Fallback: create with empty description
                            self.commands[cmd_name]['subcommands'][subcmd_name] = {
                                'description': '',
                                'parameters': []
                            }
                
                if aliases:
                    self.commands[cmd_name]['aliases'] = aliases
                
                # Add command support flags
                self.commands[cmd_name]['supports_prefix'] = supports_prefix
                self.commands[cmd_name]['supports_slash'] = supports_slash
    
    def save_to_json(self, output_path: str = "test_results/discovered_commands.json"):
        """Save discovered commands to JSON file."""
        os.makedirs(os.path.dirname(output_path), exist_ok=True)
        with open(output_path, 'w') as f:
            json.dump(self.commands, f, indent=2)
        return output_path

def main():
    """Main entry point for command extraction."""
    extractor = CommandExtractor()
    commands = extractor.extract_commands()
    
    # Print summary
    print(f"Discovered {len(commands)} top-level commands:")
    for cmd_name, cmd_info in commands.items():
        subcount = len(cmd_info.get('subcommands', {}))
        if subcount > 0:
            print(f"  /{cmd_name} - {cmd_info.get('description', 'No description')} ({subcount} subcommands)")
            for subcmd_name, subcmd_info in cmd_info.get('subcommands', {}).items():
                print(f"    - {subcmd_name}: {subcmd_info.get('description', 'No description')}")
        else:
            print(f"  /{cmd_name} - {cmd_info.get('description', 'No description')}")
    
    # Save to file
    output_path = extractor.save_to_json()
    print(f"\nCommands saved to: {output_path}")
    
    return commands

if __name__ == "__main__":
    main()