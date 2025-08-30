# Booster Role Dominant Color Command Implementation Roadmap

## Feature Summary

**Command**: `/boosterrole dominant`  
**Purpose**: Automatically set a booster's role color to match the dominant color extracted from their Discord avatar  
**Framework**: Poise (Discord bot framework for Rust)  
**Target Users**: Server boosters only  

## Discord Interaction Flow Analysis

### Command Execution Flow
1. **Slash Command Invocation**
   - User executes `/boosterrole dominant` as Discord slash command
   - Poise framework intercepts and routes to command handler

2. **Permission Verification**
   - Check if user has booster status in guild
   - Verify booster role exists and is manageable
   - Validate bot has appropriate permissions

3. **Avatar Processing**
   - Fetch user's avatar URL from Discord CDN
   - Download avatar image data
   - Process image to extract dominant color

4. **Role Update**
   - Update booster role color via Discord API
   - Handle rate limits appropriately

5. **Response Generation**
   - Create embed with color preview
   - Display hex code and RGB values
   - Show success/error status

### Error Handling Scenarios
- Non-booster user attempts command
- User has no avatar set
- Image processing failure
- Discord API rate limits
- Insufficient bot permissions
- Role hierarchy conflicts

## Module Architecture

### Primary Module
- **Path**: `src/commands/boosterrole/mod.rs`
  - Main module exporting all boosterrole subcommands
  - Module structure for extensibility

### Subcommand Module
- **Path**: `src/commands/boosterrole/dominant.rs`
  - Contains `dominant` subcommand implementation
  - Image processing logic
  - Color extraction algorithm
  - Discord API interaction

### Utility Modules to Enhance
- **Path**: `src/utils/image_processor.rs` (new)
  - Avatar fetching functionality
  - Image decoding and processing
  - Dominant color extraction algorithm
  
- **Path**: `src/utils/role_manager.rs` (existing)
  - Extend with booster-specific role operations
  - Add color update functionality

## Dependencies Analysis

### Required New Dependencies
```toml
# Add to Cargo.toml
image = "0.24"           # Image processing and decoding
palette = "0.7"          # Color extraction and manipulation
reqwest = { version = "0.11", features = ["rustls-tls"] }  # HTTP client for avatar download
```

### Existing Dependencies Utilized
- `serenity` - Discord API interactions
- `poise` - Command framework
- `tokio` - Async runtime
- `tracing` - Logging infrastructure

## Git Branch Strategy
```bash
git checkout -b feature/boosterrole-dominant-color
```

## Logging Specifications

### Command Invocation Logging
- **Location**: `src/commands/boosterrole/dominant.rs`
- **Level**: INFO
- **Format**: `tracing::info!("Boosterrole dominant command invoked by user {} in guild {}", user_id, guild_id)`

### Permission Check Logging
- **Location**: `src/commands/boosterrole/dominant.rs`
- **Level**: WARN/DEBUG
- **Format**: 
  - Success: `tracing::debug!("User {} confirmed as booster", user_id)`
  - Failure: `tracing::warn!("Non-booster {} attempted dominant color command", user_id)`

### Image Processing Logging
- **Location**: `src/utils/image_processor.rs`
- **Level**: DEBUG/ERROR
- **Format**:
  - Start: `tracing::debug!("Fetching avatar for user {}: {}", user_id, avatar_url)`
  - Success: `tracing::debug!("Dominant color extracted: #{:06X}", color)`
  - Error: `tracing::error!("Avatar processing failed for user {}: {}", user_id, error)`

### Role Update Logging
- **Location**: `src/commands/boosterrole/dominant.rs`
- **Level**: INFO/ERROR
- **Format**:
  - Success: `tracing::info!("Updated booster role color for user {} to #{:06X}", user_id, color)`
  - Error: `tracing::error!("Failed to update role for user {}: {}", user_id, error)`

## Implementation Approach Hypothesis

### Algorithm Selection
**K-means clustering approach**:
1. Decode avatar image into RGB pixel array
2. Apply K-means clustering (k=5) to identify color clusters
3. Select cluster with most pixels as dominant
4. Convert to Discord-compatible hex format

**Alternative: Color histogram approach**:
- Build color histogram with buckets
- Find most frequent color range
- Calculate average of that range

### Performance Considerations
- Cache processed avatars for 5 minutes
- Limit image size to 256x256 for processing
- Use async/await for non-blocking operations
- Implement timeout for image downloads (5 seconds)

## Step-by-Step Implementation Roadmap

### Phase 1: Foundation Setup
1. **Create branch**: `git checkout -b feature/boosterrole-dominant-color`
2. **Update dependencies**: Add `image`, `palette`, and `reqwest` to `Cargo.toml`
3. **Create module structure**:
   - Create `src/commands/boosterrole/` directory
   - Create `src/commands/boosterrole/mod.rs`
   - Create `src/commands/boosterrole/dominant.rs`

### Phase 2: Utility Development
4. **Create image processor utility**:
   - Create `src/utils/image_processor.rs`
   - Implement `fetch_avatar()` function
   - Implement `extract_dominant_color()` function
   - Add module export to `src/utils/mod.rs`

5. **Enhance role manager**:
   - Update `src/utils/role_manager.rs`
   - Add `update_booster_role_color()` method
   - Add booster status verification helpers

### Phase 3: Command Implementation
6. **Implement dominant subcommand**:
   - Define command with `#[poise::command()]` macro
   - Add booster permission check
   - Integrate avatar fetching
   - Process image for dominant color
   - Update role via Discord API

7. **Create parent boosterrole command**:
   - Define parent command structure in `mod.rs`
   - Configure subcommand routing
   - Export from `src/commands/mod.rs`

### Phase 4: Integration
8. **Register command in framework**:
   - Update `src/bot/framework.rs`
   - Add `boosterrole::boosterrole()` to commands vector
   - Ensure proper initialization

9. **Database schema (if needed)**:
   - Consider adding color history table
   - Cache avatar processing results

### Phase 5: Testing & Verification
10. **Local testing**:
    - Test with various avatar types (PNG, JPG, GIF, WebP)
    - Test with no avatar scenario
    - Test permission edge cases
    - Verify embed responses

11. **Discord interaction testing**:
    - Deploy to test server
    - Verify slash command registration
    - Test with actual boosters
    - Monitor rate limits

### Phase 6: Error Handling & Polish
12. **Comprehensive error handling**:
    - Handle network timeouts
    - Process invalid image formats
    - Manage Discord API errors
    - Provide user-friendly error messages

13. **Performance optimization**:
    - Implement caching layer
    - Add progress indicators for long operations
    - Optimize image processing algorithm

### Phase 7: Documentation
14. **Code documentation**:
    - Add rustdoc comments
    - Document algorithm choices
    - Explain permission requirements

15. **User documentation**:
    - Update bot help command
    - Add usage examples
    - Document limitations

## Success Criteria

1. ✅ Command appears in Discord slash command menu
2. ✅ Only boosters can execute the command
3. ✅ Avatar is successfully fetched and processed
4. ✅ Dominant color is accurately extracted
5. ✅ Role color updates immediately
6. ✅ Embed response shows color preview
7. ✅ Appropriate error messages for all failure cases
8. ✅ Command respects Discord rate limits
9. ✅ Logging provides debugging information
10. ✅ Performance under 3 seconds for typical avatars

## Risk Mitigation

### Technical Risks
- **Image format incompatibility**: Use robust image library with format detection
- **Large avatar files**: Implement size limits and timeouts
- **Color extraction accuracy**: Test multiple algorithms, allow user override

### Discord API Risks
- **Rate limiting**: Implement exponential backoff
- **Permission conflicts**: Comprehensive permission checks
- **Role hierarchy issues**: Validate bot role position

### User Experience Risks
- **Slow processing**: Add "Processing..." feedback
- **Unclear errors**: Detailed, actionable error messages
- **Color mismatch expectations**: Show extracted color clearly

## Future Enhancements (Out of Scope)

- Manual color override option
- Color palette extraction (multiple colors)
- Animated avatar frame selection
- Color history tracking
- Scheduled color updates
- Guild-wide booster color themes

## Notes

- The existing `boosterrole` module reference in `src/commands/mod.rs` suggests infrastructure is partially prepared
- Leverage existing `EmbedBuilder` and `ResponseHelper` for consistent UI
- Follow established error handling patterns with `BotError` enum
- Maintain async/await patterns throughout implementation
- Consider adding feature flag for experimental color algorithms