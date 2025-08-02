# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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