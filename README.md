# 🚀 Fade Launcher

A sleek, modern application launcher for Windows built with Rust and featuring a stunning pink-to-aqua gradient design. Fade Launcher provides advanced search capabilities that go beyond traditional file browsers like "Everything" with a beautiful, animated user interface.

## ✨ Features

### 🎨 Beautiful UI Design
- **Gradient Background**: Stunning animated gradient from hot pink to aqua blue
- **Floating Particles**: Subtle animated particles for extra visual flair
- **Smooth Animations**: Fluid transitions and responsive interface
- **Translucent Windows**: Modern glass-like appearance with transparency
- **Borderless Design**: Clean, minimal window decorations

### 🔍 Advanced Search
- **Intelligent Indexing**: Scans multiple Windows directories for applications
- **Fuzzy Search**: Find apps even with partial or misspelled queries
- **Relevance Scoring**: Results ranked by relevance and usage frequency
- **Real-time Results**: Instant search results as you type
- **Recent Apps**: Quick access to recently launched applications

### ⚡ Performance
- **Background Scanning**: Non-blocking app discovery
- **Fast Launch**: Quick application startup
- **Memory Efficient**: Minimal resource usage
- **Threaded Architecture**: Responsive UI even during intensive operations

### 🛠️ Customization
- **Configurable Paths**: Add custom directories to scan
- **File Type Filtering**: Choose which file types to include
- **Hotkey Support**: Keyboard shortcuts for quick access
- **Theme Options**: Multiple appearance configurations

## 🎯 Supported File Types

- `.exe` - Executable files
- `.lnk` - Windows shortcuts
- `.bat` - Batch files
- `.cmd` - Command files
- `.msi` - Windows installer packages
- `.com` - DOS executables

## 📂 Scanned Directories

Fade Launcher automatically scans common Windows application directories:

- `Program Files` and `Program Files (x86)`
- Windows Apps (`%LocalAppData%\Microsoft\WindowsApps`)
- Start Menu Programs
- Desktop shortcuts
- System PATH directories

## 🚀 Getting Started

### Prerequisites

1. **Install Rust**: Download and install from [rustup.rs](https://rustup.rs/)
2. **Windows 10/11**: Required for Windows-specific APIs

### Building from Source

```bash
# Clone the repository
git clone <your-repo-url>
cd fade-launcher

# Build the project
cargo build --release

# Run the application
cargo run --release
```

### First Run

1. Launch Fade Launcher
2. Wait for initial app scanning to complete
3. Start typing to search for applications
4. Press `Enter` or click to launch selected app

## ⌨️ Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl + Space` | Toggle launcher (planned) |
| `Escape` | Clear search or close |
| `↑` `↓` | Navigate results |
| `Enter` | Launch selected app |
| `Ctrl + ,` | Open settings |

## 🎨 Color Scheme

The Fade theme features a carefully crafted color palette:

- **Pink Primary**: `#FF69B4` (Hot Pink)
- **Pink Secondary**: `#FFB6C1` (Light Pink)  
- **Aqua Primary**: `#00FFFF` (Cyan/Aqua)
- **Aqua Secondary**: `#7FFFD4` (Aquamarine)
- **Gradient Mid**: `#7FB4D9` (Purple-blue mix)

## 🏗️ Architecture

### Core Modules

- **`main.rs`**: Application entry point and window setup
- **`app.rs`**: Main application logic and state management
- **`ui.rs`**: User interface rendering and animations
- **`search.rs`**: Application discovery and search algorithms
- **`theme.rs`**: Color constants and gradient functions
- **`config.rs`**: Configuration management and persistence

### Dependencies

- **egui/eframe**: Modern immediate mode GUI framework
- **tokio**: Async runtime for background operations
- **walkdir**: Recursive directory traversal
- **serde**: Configuration serialization
- **windows**: Windows-specific API bindings

## 🔧 Configuration

Configuration is stored in `%AppData%\fade-launcher\config.json`:

```json
{
  "ui": {
    "window_width": 800.0,
    "window_height": 600.0,
    "always_on_top": true,
    "hide_on_launch": true
  },
  "search": {
    "max_results": 10,
    "file_extensions": [".exe", ".lnk", ".bat"],
    "exclude_patterns": ["unins", "setup"]
  },
  "appearance": {
    "theme_variant": "fade",
    "transparency": 0.95,
    "gradient_animation": true
  }
}
```

## 🤝 Contributing

Contributions are welcome! Here are some areas where you can help:

- **Icon Extraction**: Add support for displaying application icons
- **Global Hotkeys**: Implement system-wide keyboard shortcuts  
- **Theme Variants**: Create additional color schemes
- **Performance**: Optimize search algorithms and UI rendering
- **Features**: Add new functionality like app categories, favorites, etc.

## 📋 TODO

- [ ] Extract and display application icons
- [ ] Implement global hotkey registration
- [ ] Add application categories and filtering
- [ ] Create settings UI for configuration
- [ ] Support for additional file types
- [ ] Plugin system for extensibility
- [ ] Auto-updater functionality
- [ ] Portable mode option

## 🐛 Known Issues

- Global hotkeys not yet implemented
- Application icons not displayed
- Some Windows Store apps may not be detected
- Initial scanning can take time on first run

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [egui](https://github.com/emilk/egui) - Amazing immediate mode GUI
- Inspired by launchers like Alfred, Raycast, and PowerToys Run
- Color palette designed for maximum visual appeal and usability

---

**Made with ❤️ and Rust** 🦀