# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Development Commands

### Building and Running
```bash
# Build the project in debug mode
cargo build

# Build optimized release version
cargo build --release

# Run the application in debug mode
cargo run

# Run the optimized release version
cargo run --release
```

### Development Tasks
```bash
# Check for compilation errors without building
cargo check

# Run formatting
cargo fmt

# Run linting
cargo clippy

# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Generate documentation
cargo doc --open
```

### Testing Commands
Currently no tests exist in the project. When adding tests:
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name
```

## Architecture Overview

Fade Launcher is a Rust-based Windows application launcher built with the egui immediate mode GUI framework. The application features a pink-to-aqua gradient theme with smooth animations and advanced fuzzy search capabilities.

### Core Module Structure

**`main.rs`** - Application entry point
- Sets up the egui window with transparency and borderless design
- Configures custom fonts and dark theme styling
- Initializes the main FadeLauncher app

**`app.rs`** - Main application state and logic
- `FadeLauncher` struct manages the overall application state
- Handles search queries, results, and application launching
- Manages UI state including settings dialog and keyboard shortcuts
- Implements the main egui App trait with update loop

**`search.rs`** - Application discovery and indexing
- `AppSearcher` handles background scanning of Windows directories
- Scans common locations: Program Files, Start Menu, Desktop, PATH
- Supports multiple file types: .exe, .lnk, .bat, .cmd, .msi, .com
- Implements fuzzy search with relevance scoring
- Maintains recent applications list

**`ui.rs`** - User interface rendering and animations
- `LauncherUI` handles all visual rendering
- Creates animated gradient backgrounds using vertex meshes
- Draws floating particle effects for visual enhancement
- Manages search box, results list, and keyboard navigation
- Handles click and keyboard interactions

**`theme.rs`** - Color scheme and visual constants
- `FadeTheme` defines the pink-to-aqua gradient color palette
- Provides color interpolation functions for smooth gradients
- Defines UI element colors (backgrounds, text, borders)
- Includes helper functions for alpha blending and glow effects

**`config.rs`** - Configuration management
- Serializable configuration structures for all app settings
- Handles loading/saving to `%APPDATA%\fade-launcher\config.json`
- Organized into UI, Search, Hotkey, and Appearance configurations
- Provides default values and validation for settings

### Key Design Patterns

**Threaded Architecture**: Background app scanning runs in separate threads to keep UI responsive during intensive file system operations.

**Immediate Mode GUI**: Uses egui's immediate mode pattern where UI is rebuilt every frame, enabling smooth animations and dynamic layouts.

**Gradient Rendering**: Custom vertex mesh generation creates animated gradients by interpolating colors across a grid of vertices.

**Fuzzy Search**: String matching with relevance scoring allows finding applications even with partial or misspelled queries.

**Configuration System**: JSON-based configuration with automatic defaults and validation ensures user settings persist across sessions.

### Windows-Specific Integration

The application leverages Windows-specific APIs through the `windows` crate:
- Registry access for application discovery
- Shell integration for launching applications
- File system APIs for scanning directories
- Windows-specific directory paths (Program Files, Start Menu, etc.)

### Animation System

The UI features a time-based animation system:
- Gradient colors shift smoothly using sine wave functions
- Particle effects move in organic patterns
- All animations are frame-rate independent using delta time

## Development Guidelines

### Code Organization
- Keep UI rendering logic in `ui.rs` 
- Business logic and state management belongs in `app.rs`
- All color constants and theme functions go in `theme.rs`
- Platform-specific code should be properly feature-gated with `#[cfg(windows)]`

### Performance Considerations
- Background operations must use threading to avoid blocking the UI
- Limit file system scanning depth to prevent excessive resource usage
- Use efficient data structures for search indexing (consider caching)
- Animation calculations should be lightweight for smooth 60fps rendering

### Error Handling
- Use proper error handling for file system operations
- Log errors to stderr using env_logger (set RUST_LOG=debug for verbose output)
- Gracefully handle missing directories or inaccessible files during scanning

### Windows Compatibility
- Target Windows 10/11 for modern API support
- Handle different Windows directory structures (Program Files vs Program Files (x86))
- Support both traditional Win32 applications and modern Windows Store apps

### UI/UX Guidelines
- Maintain the pink-to-aqua gradient theme throughout the interface
- Ensure keyboard navigation works smoothly (arrow keys, Enter, Escape)
- Provide visual feedback for hover states and selections
- Keep animations subtle and performance-friendly