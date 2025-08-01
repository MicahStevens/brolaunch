# brolaunch

A smart Chromium launcher that automatically opens URLs with the right profile based on regex pattern matching, with GUI profile selection and app mode support.

## Features

- 🎯 **Automatic profile matching** - URLs are matched against regex patterns to select the appropriate Chromium profile
- 🖥️ **GUI profile chooser** - When no patterns match, a clean GUI lets you choose from available profiles
- 📱 **App mode support** - Launch URLs as dedicated app windows (no address bar, tabs, etc.)
- 🔧 **Configurable** - Flexible YAML configuration with per-profile settings
- 🗂️ **Custom user data directories** - Each profile can have its own isolated data directory
- 📝 **Verbose logging** - Debug mode shows detailed execution information

## Installation

### Prerequisites

- Rust (latest stable)
- Chromium or Google Chrome or other chromium based browsers with compatible CLI flags. 
- FLTK dependencies for GUI (on most Linux distros, install `fltk-dev` or similar)

### Build and Install

```bash
# Clone and build
git clone <repository-url>
cd brolaunch
cargo build --release

# Copy binary to PATH
cp target/release/brolaunch ~/.local/bin/

# Copy default config
mkdir -p ~/.config
cp config.yaml ~/.config/brolaunch.yaml
```

Or use the custom install script:
```bash
chmod +x cargo-install-brolaunch
./cargo-install-brolaunch
```

## Usage

### Basic Usage

```bash
# Launch with URL (auto-selects profile based on patterns)
brolaunch https://github.com/mycompany/repo

# Launch specific profile without URL
brolaunch work

# Launch default profile (if configured)
brolaunch

# Launch URL as dedicated app window
brolaunch --app https://gmail.com

# Verbose mode (shows detailed execution info)
brolaunch -v https://example.com

# Use custom config file
brolaunch -c /path/to/config.yaml https://example.com
```

### Command Line Options

| Option | Description |
|--------|-------------|
| `<URL_OR_PROFILE>` | URL to open or profile name to launch (optional if default_profile is set) |
| `-c, --config <FILE>` | Path to config file (default: `~/.config/brolaunch.yaml` or `./config.yaml`) |
| `-v, --verbose` | Enable verbose logging (shows config file used, pattern matching, command execution) |
| `--app` | Launch URL as an app window (creates app-like window without browser UI) |
| `-h, --help` | Show help message |
| `-V, --version` | Show version |

## Configuration

### Config File Locations

brolaunch looks for configuration files in this order:

1. Path specified with `-c/--config` flag
2. `~/.config/brolaunch.yaml` 
3. `./config.yaml` in current directory

### Configuration Syntax

```yaml
# Path to Chromium binary (optional, defaults to "chromium")
# Use direct binary path to avoid wrapper script issues
chromium_binary: "/usr/lib/chromium/chromium"  # Arch Linux
# chromium_binary: "/usr/bin/chromium-browser"  # Ubuntu/Debian
# chromium_binary: "/usr/bin/google-chrome"     # Google Chrome

# Default profile to use when no arguments provided (optional)
default_profile: "Personal"

# Profile configurations
profiles:
  Work:
    # Custom user data directory for this profile (optional)
    user_data_dir: "/home/user/.config/brolaunch/chromium-work"
    
    # Default app mode for this profile (optional, defaults to false)
    app_mode: false
    
    # URL patterns that open in regular browser windows
    patterns:
      - "github\\.com/company"
      - "gitlab\\.company\\.com"
      - "bitbucket\\.org"
      - "\\.atlassian\\.net"
      - "jira\\."
      - "confluence\\."
    
    # URL patterns that automatically open as app windows
    app_patterns:
      - "company\\.slack\\.com"
      - "trello\\.com"
      - "\\.atlassian\\.net/jira"

  Personal:
    user_data_dir: "/home/user/.config/brolaunch/chromium-personal"
    patterns:
      - "reddit\\.com"
      - "youtube\\.com"
      - "twitter\\.com"
    app_patterns:
      - "gmail\\.com"
      - "mail\\.google\\.com"
      - "calendar\\.google\\.com"
      - "drive\\.google\\.com"
```

### Configuration Options

#### Global Options

| Option | Type | Description | Default |
|--------|------|-------------|---------|
| `chromium_binary` | string | Path to Chromium/Chrome binary | `"chromium"` |
| `default_profile` | string | Profile to use when no arguments provided | none |

#### Profile Options

| Option | Type | Description | Default |
|--------|------|-------------|---------|
| `user_data_dir` | string | Custom user data directory for this profile | Chromium default |
| `app_mode` | boolean | Default app mode for this profile's regular patterns | `false` |
| `patterns` | array | Regex patterns for URLs that open in browser windows | none |
| `app_patterns` | array | Regex patterns for URLs that open as app windows | none |

### Pattern Matching

- **Regex patterns** - All patterns use standard regex syntax
- **App patterns priority** - `app_patterns` are checked before `patterns`
- **Case sensitive** - Patterns are case-sensitive by default
- **Escape special characters** - Use `\\.` for literal dots, `\\` for literal backslashes

#### Pattern Examples

```yaml
patterns:
  - "github\\.com"           # Matches github.com
  - ".*\\.company\\.com"     # Matches any subdomain of company.com
  - "jira\\."                # Matches jira.anything
  - "localhost:[0-9]+"       # Matches localhost with any port
  - "192\\.168\\."           # Matches 192.168.x.x addresses
```

### App Mode vs Window Mode

| Mode | Use Case | UI | Behavior |
|------|----------|----| ---------|
| **Window Mode** | General browsing | Full browser (address bar, tabs, bookmarks) | Opens in new window, can have multiple tabs |
| **App Mode** | Web applications | Minimal UI (no address bar, no tabs) | Dedicated app-like window, single purpose |

**App Mode is perfect for:**
- Gmail, Calendar, Drive (Google Workspace)
- Slack, Teams, Discord (Communication)
- Trello, Notion, Asana (Productivity)
- Jira, Confluence (Atlassian tools)

## Examples

### Example Workflows

```bash
# Open work email as dedicated app
brolaunch https://mail.company.com
# → Matches Work profile app_patterns, opens as app

# Browse GitHub in work profile  
brolaunch https://github.com/company/repo
# → Matches Work profile patterns, opens in browser window

# Quick access to work profile
brolaunch work
# → Opens Work profile without URL

# Force app mode for any URL
brolaunch --app https://notion.so
# → Opens in app mode regardless of patterns

# Debug pattern matching
brolaunch -v https://example.com
# → Shows which config file used, pattern matching results, command executed
```

### Multi-Profile Setup

```yaml
profiles:
  Work:
    user_data_dir: "/home/user/.config/brolaunch/work"
    patterns:
      - "github\\.com/company"
      - "\\.company\\.com"
    app_patterns:
      - "company\\.slack\\.com"
      - "company\\.atlassian\\.net"

  Personal:
    user_data_dir: "/home/user/.config/brolaunch/personal"  
    patterns:
      - "reddit\\.com"
      - "youtube\\.com"
    app_patterns:
      - "gmail\\.com"
      - "calendar\\.google\\.com"

  Client:
    user_data_dir: "/home/user/.config/brolaunch/client"
    patterns:
      - "client\\.com"
      - "clientproject\\."
```

## Troubleshooting

### Common Issues

**Duplicate command line flags:**
```bash
# Problem: Using wrapper script
chromium_binary: "chromium"

# Solution: Use direct binary
chromium_binary: "/usr/lib/chromium/chromium"  # Arch
chromium_binary: "/usr/bin/chromium-browser"   # Ubuntu
```

**GUI window not floating in tiling WM:**
- The profile chooser uses modal windows which should float automatically
- For Hyprland/other Wayland compositors, modal windows typically float by default

**App mode not working:**
- Ensure you're using `--app` flag or `app_patterns` in config
- Some websites may not work well in app mode

**Profile not found:**
- Use `-v` flag to see available profiles
- Check profile names are exact matches (case-sensitive)

### Debug Mode

Use `-v/--verbose` to see detailed execution information:

```bash
brolaunch -v https://example.com
```

Output includes:
- Config file path used
- Available profiles and pattern counts  
- Pattern matching results
- Final command executed
- Profile and user data directory used

## License

MIT License - see LICENSE file for details.