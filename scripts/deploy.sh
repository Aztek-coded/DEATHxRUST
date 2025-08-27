#!/bin/bash

# Discord Bot Slash Command Deployment Script
# ==========================================

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Default values
DEPLOYMENT_TYPE=""
GUILD_ID=""
CONFIRM=false
DRY_RUN=false

# Functions
print_usage() {
    echo "Discord Bot Slash Command Deployment Script"
    echo
    echo "USAGE:"
    echo "    $0 [OPTIONS]"
    echo
    echo "OPTIONS:"
    echo "    -g, --global                Deploy commands globally"
    echo "    -u, --guild GUILD_ID        Deploy commands to specific guild"
    echo "    -y, --yes                   Skip confirmation prompt"
    echo "    -d, --dry-run              Show what would be deployed without deploying"
    echo "    -h, --help                 Show this help message"
    echo
    echo "EXAMPLES:"
    echo "    $0 --global                # Deploy globally"
    echo "    $0 --guild 123456789       # Deploy to specific guild"
    echo "    $0 --dry-run --global      # Preview global deployment"
    echo "    $0                         # Use environment configuration"
    echo
    echo "ENVIRONMENT VARIABLES:"
    echo "    DISCORD_TOKEN              Required: Your bot token"
    echo "    DEVELOPMENT_GUILD_ID       Optional: Default guild for deployment"
    echo "    SLASH_COMMANDS_GLOBAL      Optional: Deploy globally by default (true/false)"
}

print_banner() {
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE}Discord Slash Command Deployment${NC}"
    echo -e "${BLUE}================================${NC}"
    echo
}

check_dependencies() {
    echo -e "${YELLOW}Checking dependencies...${NC}"
    
    # Check if cargo is available
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}❌ Cargo not found. Please install Rust.${NC}"
        exit 1
    fi
    
    # Check if .env file exists or required env vars are set
    if [[ ! -f "$PROJECT_ROOT/.env" ]] && [[ -z "$DISCORD_TOKEN" ]]; then
        echo -e "${RED}❌ DISCORD_TOKEN not set and no .env file found.${NC}"
        echo "Please set DISCORD_TOKEN environment variable or create a .env file."
        exit 1
    fi
    
    echo -e "${GREEN}✅ Dependencies OK${NC}"
    echo
}

validate_guild_id() {
    if [[ ! "$1" =~ ^[0-9]{17,20}$ ]]; then
        echo -e "${RED}❌ Invalid guild ID: $1${NC}"
        echo "Guild IDs should be 17-20 digits long."
        exit 1
    fi
}

confirm_deployment() {
    if [[ "$CONFIRM" == true ]]; then
        return 0
    fi
    
    echo -e "${YELLOW}⚠️  Deployment Summary:${NC}"
    if [[ "$DEPLOYMENT_TYPE" == "global" ]]; then
        echo -e "   ${BLUE}Target:${NC} Global (all servers)"
        echo -e "   ${BLUE}Time:${NC} Up to 1 hour to propagate"
    elif [[ -n "$GUILD_ID" ]]; then
        echo -e "   ${BLUE}Target:${NC} Guild $GUILD_ID"
        echo -e "   ${BLUE}Time:${NC} Immediate"
    else
        echo -e "   ${BLUE}Target:${NC} Using environment configuration"
        echo -e "   ${BLUE}Time:${NC} Varies based on config"
    fi
    echo -e "   ${BLUE}Action:${NC} Clear existing commands and deploy new ones"
    echo
    
    read -p "Do you want to continue? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo -e "${YELLOW}Deployment cancelled.${NC}"
        exit 0
    fi
}

build_project() {
    echo -e "${YELLOW}Building project...${NC}"
    cd "$PROJECT_ROOT"
    
    if ! cargo build --bin deploy_commands --release; then
        echo -e "${RED}❌ Build failed${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✅ Build successful${NC}"
    echo
}

run_deployment() {
    echo -e "${YELLOW}Starting deployment...${NC}"
    cd "$PROJECT_ROOT"
    
    local deploy_args=()
    
    if [[ "$DEPLOYMENT_TYPE" == "global" ]]; then
        deploy_args+=("--global")
    elif [[ -n "$GUILD_ID" ]]; then
        deploy_args+=("--guild" "$GUILD_ID")
    fi
    
    if [[ "$DRY_RUN" == true ]]; then
        echo -e "${BLUE}DRY RUN: Would execute:${NC}"
        echo "cargo run --bin deploy_commands --release ${deploy_args[*]}"
        echo
        echo -e "${YELLOW}This would deploy the following commands:${NC}"
        echo "  /ping - Check if the bot is responsive"
        echo "  /help - Show available commands and their usage"
        echo "  /info - Get information about the server, a user, or the bot"
        return 0
    fi
    
    if cargo run --bin deploy_commands --release -- "${deploy_args[@]}"; then
        echo
        echo -e "${GREEN}🎉 Deployment successful!${NC}"
        
        if [[ "$DEPLOYMENT_TYPE" == "global" ]]; then
            echo -e "${YELLOW}⏳ Global commands may take up to 1 hour to appear in all servers.${NC}"
        else
            echo -e "${GREEN}⚡ Commands should be available immediately.${NC}"
        fi
    else
        echo -e "${RED}❌ Deployment failed${NC}"
        exit 1
    fi
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -g|--global)
            DEPLOYMENT_TYPE="global"
            shift
            ;;
        -u|--guild)
            GUILD_ID="$2"
            validate_guild_id "$GUILD_ID"
            shift 2
            ;;
        -y|--yes)
            CONFIRM=true
            shift
            ;;
        -d|--dry-run)
            DRY_RUN=true
            shift
            ;;
        -h|--help)
            print_usage
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            print_usage
            exit 1
            ;;
    esac
done

# Main execution
main() {
    print_banner
    check_dependencies
    
    if [[ "$DRY_RUN" != true ]]; then
        confirm_deployment
        build_project
    fi
    
    run_deployment
    
    echo
    echo -e "${GREEN}✨ Deployment process complete!${NC}"
}

# Run main function
main