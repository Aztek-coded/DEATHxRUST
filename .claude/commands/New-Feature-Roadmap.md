# Task: Create Discord Bot Feature Implementation Roadmap

## Instructions

1. **Analyze Feature Details**: Review the provided command name, intended functionality, and any Discord-specific behaviors. Identify core requirements, user interactions (slash commands, messages, embeds), event flows, and integration points with existing bot modules.

2. **Analyze Development Guidelines**: Study the Rust Discord bot guidelines. Ensure all steps align with Poise framework patterns, async/await handling, error management, and project architecture standards.

3. **Analyze Required Reference Files**: Examine listed filepaths, modules, command handlers, and code patterns. Map them to the feature to identify reuse opportunities (e.g., existing command patterns, utility functions, data structures).

4. **Analyze Optional Reference Files (if necessary)**: Only if required for clarity, review additional modules or dependencies. Justify usage in analysis.

5. **Perform Implementation Analysis**:
   - Hypothesize approaches: Break down into command handlers, event processors, data models, and Discord API interactions
   - Trace from Discord events to bot responses: E.g., slash command → parse arguments → execute logic → send embed/message
   - Consider async execution, rate limits, permissions, and error handling
   - Prioritize modular changes following the bot's module structure

6. **Develop Roadmap Strategy**:
   - Think deeply about Discord bot patterns and Poise framework requirements
   - Outline step-by-step implementation: 
     - Specify modules, commands, handlers to modify/add (e.g., `src/commands/new_feature.rs`)
     - Git branch creation (e.g., `git checkout -b feature/music-commands`)
     - Logging additions using `tracing` (where, what to log: e.g., command invocations, API responses, errors)
   - Include verification via bot logging and Discord interactions

7. **Identify Key Changes**:
   - Summarize main updates: E.g., new command module, event handler, data structures for Discord entities
   - Consider command registration, permissions, and guild vs global deployment
   - Prioritize based on impact and Discord API constraints

8. **Output Roadmap Plan**:
   - Create a markdown file in the `/Users/aztek/Desktop/DEATHxRUST/.claude/roadmaps` directory
   - Include: 
     - Feature/command summary
     - Discord interaction flow analysis
     - Explicit modules/commands/handlers with filepaths
     - Branch details
     - Logging specifications using `tracing`
     - Hypothesized implementation approaches
     - Step-by-step roadmap with Poise/Serenity considerations
   - Do not implement code or logic
   - Ensure plan follows Rust async patterns and Discord bot best practices

## Feature Details:

### Command/Feature Name:
**Dual Color Avatar Extraction for Boosterrole Dominant Command**

### Intended Functionality:
- Command type (slash/prefix/both): **Enhancement to existing slash/prefix command**
- Description: Enhance existing `boosterrole dominant` command to extract **top 2 dominant colors** from user's avatar for dual color roles
- Arguments/options: **None** - Command automatically extracts dual colors from user's avatar
- Permissions required: **Booster Only** (user must have Nitro boost status in the guild)

### Discord Interactions:
- Message types (text/embed/buttons/modals): **Enhanced embeds** showing both primary and secondary colors with dual previews
- Event handling requirements: 
  - Enhanced avatar image processing for dual color extraction
  - Dual color role application via Discord API
  - Backward compatibility with existing single color functionality
- Expected user flow:
  1. User types `/boosterrole dominant`
  2. Bot verifies user has booster status
  3. Bot fetches user's avatar image
  4. Bot analyzes avatar to extract **top 2 dominant colors**
  5. Bot updates user's booster role with dual colors (primary + secondary)
  6. Bot responds with enhanced embed showing both colors

### Symptoms/Behaviors (if updating existing feature):
- **Current Behavior**: Single dominant color extraction using k-means clustering (k=5) at `src/utils/image_processor.rs:48`
- **Current Implementation**: Returns single `u32` color value from `extract_dominant_color` function
- **Limitation**: Single color roles don't represent complex avatars with multiple prominent colors

### Expected Outcomes:
1. **Dual Color Extraction**: Extract and return top 2 most prominent colors from avatar analysis
2. **Enhanced Role Appearance**: Booster roles display with both primary and secondary colors
3. **Backward Compatibility**: Existing single color functionality remains intact
4. **Improved Avatar Representation**: Complex avatars with multiple colors better represented
5. **Enhanced Visual Feedback**: Embed shows both colors with hex codes and dual preview
6. **Discord API Integration**: Leverage Discord's new dual color role capabilities





## Reference Files:

### Required Files to Analyze:
- `/src/commands/boosterrole/dominant.rs` - Existing dominant color command implementation
- `/src/commands/boosterrole/mod.rs` - Boosterrole command module structure
- `/src/utils/image_processor.rs` - Current color extraction algorithms
- `/src/bot/framework.rs` - Poise framework setup and command registration

### Optional Files (if needed):
- `/src/utils/error.rs` - Error handling patterns for image processing
- `/src/commands/boosterrole/color.rs` - Related color command for pattern reference
- Existing roadmap: `.claude/roadmaps/boosterrole-dominant-color-roadmap.md`

## Reference Screenshots (if applicable):







## Development Guidelines:

Primary: `/Users/aztek/Desktop/DEATHxRUST/CLAUDE.md`
Secondary: `/Users/aztek/Desktop/DEATHxRUST/.claude/guidelines/rust-discord-bot-guidelines.md` (if exists)

