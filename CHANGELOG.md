# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-11-02

### Added
- Firefox browser support alongside Chromium
- `browser_type` configuration option to choose between Chromium and Firefox
- `firefox_binary` configuration option for custom Firefox binary paths
- Firefox profile support using `-profile` flag or profile names with `-P`
- App mode warnings for Firefox (since app mode is Chromium-only)
- Enhanced verbose logging to show browser type and Firefox-specific information
- **Hyprland integration**: Automatic window rules, workspace, and monitor management
- `hyprland_workspace` configuration option to launch browsers on specific workspaces
- `hyprland_monitor` configuration option to target specific monitors
- `hyprland_window_rules` configuration option for custom Hyprland window rules
- hyprctl IPC integration for applying window rules after browser launch
- Hyprland-specific dialog tools (Hyprpicker) with improved desktop detection
- Window class detection and rule application for better window management

### Changed
- Updated configuration schema to support browser-specific settings
- Improved error messages and verbose output for multi-browser support
- App patterns are treated as regular patterns for Firefox (app mode not supported)
- Enhanced desktop environment detection with separate Hyprland handling
- Improved dialog system with Hyprland-specific tool preferences

### Fixed
- Better handling of browser-specific command line arguments
- Improved profile directory handling for both Chromium and Firefox

## [0.1.0] - 2025-01-02

### Added
- Initial release of brolaunch
- Automatic Chromium profile selection based on URL patterns
- GUI profile chooser for unmatched URLs
- App mode support for launching web apps
- Configurable profiles with custom user data directories
- Regex pattern matching for URLs
- Verbose logging mode
- Support for multiple Chromium-based browsers
- Linux x86_64 binary releases

### Features
- Smart profile detection using regex patterns
- Clean GUI interface built with FLTK
- Separate app_patterns for automatic app mode launching
- Per-profile user data directory isolation
- Default profile configuration
- Command-line override for app mode

### Platform Support
- Linux (x86_64) - Tested and supported
- macOS - Not yet tested
- Windows - Not yet tested

[0.1.0]: https://github.com/MicahStevens/brolaunch/releases/tag/v0.1.0