# brolaunch

A smart browser launcher for Chromium and Firefox that automatically opens URLs with the right profile based on regex pattern matching, with GUI profile selection and app mode support (Chromium only).

## Features
- simple configuration file driven
- works with Firefox and Chrome
- automatically routes URL's to profile based on regex routes in config file
- unknown URLs pop up a GUI box to choose the profile

## Installation

### Install from Release (Recommended)

Download the latest release for Linux x86_64:

```bash
# Download latest release
wget https://github.com/MicahStevens/brolaunch/releases/latest/download/brolaunch-v0.2.0-x86_64-unknown-linux-gnu.tar.gz

# Extract
tar xzf brolaunch-v0.2.0-x86_64-unknown-linux-gnu.tar.gz

# Install binary
sudo cp brolaunch-*/brolaunch /usr/local/bin/
# Or for user install:
cp brolaunch-*/brolaunch ~/.local/bin/

# Copy config template
cp brolaunch-*/config.yaml ~/.config/brolaunch.yaml
```

### Build from Source

Prerequisites:
- Rust (latest stable)
- Chromium, Google Chrome, Firefox, or other Chromium-based browsers
- FLTK dependencies for GUI (on most Linux distros, install `fltk-dev` or similar)

```bash
# Clone and build
git clone https://github.com/MicahStevens/brolaunch.git
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
| `-e, --existing` | Open URL in newest existing window for the profile (if any) |
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
# Browser type: "chromium" or "firefox" (optional, defaults to "chromium")
browser_type: chromium

# Path to Chromium binary (optional, defaults to "chromium")
# Use direct binary path to avoid wrapper script issues
chromium_binary: "/usr/lib/chromium/chromium"  # Arch Linux
# chromium_binary: "/usr/bin/chromium-browser"  # Ubuntu/Debian
# chromium_binary: "/usr/bin/google-chrome"     # Google Chrome

# Path to Firefox binary (optional, defaults to "firefox")
firefox_binary: "firefox"
# firefox_binary: "/usr/bin/firefox"
# firefox_binary: "/usr/lib/firefox/firefox"

# Default profile to use when no arguments provided (optional)
default_profile: "Personal"

# Profile configurations
profiles:
  Work:
    # Custom user data directory for this profile (optional)
    # For Chromium: custom profile directory
    # For Firefox: path to Firefox profile directory (e.g., ~/.mozilla/firefox/xyz.default)
    user_data_dir: "/home/user/.config/brolaunch/chromium-work"
    
    # Default app mode for this profile (optional, defaults to false)
    # Note: app_mode is only supported for Chromium, ignored for Firefox
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
    # Note: app_patterns only work with Chromium, treated as regular patterns for Firefox
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
| `browser_type` | string | Browser to use: `"chromium"` or `"firefox"` | `"chromium"` |
| `chromium_binary` | string | Path to Chromium/Chrome binary | `"chromium"` |
| `firefox_binary` | string | Path to Firefox binary | `"firefox"` |
| `default_profile` | string | Profile to use when no arguments provided | none |

#### Profile Options

| Option | Type | Description | Default |
|--------|------|-------------|---------|
| `user_data_dir` | string | Custom user data directory (Chromium) or profile path (Firefox) | Browser default |
| `app_mode` | boolean | Default app mode for regular patterns (Chromium only) | `false` |
| `patterns` | array | Regex patterns for URLs that open in browser windows | none |
| `app_patterns` | array | Regex patterns for URLs that open as app windows (Chromium only) | none |
| `cli_flags` | array | Additional CLI flags to pass to the browser | none |
| `hyprland_workspace` | string | Workspace to launch browser on (Hyprland only) | none |
| `hyprland_monitor` | string | Monitor to launch browser on (Hyprland only) | none |
| `hyprland_window_rules` | array | Custom Hyprland window rules to apply | none |

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

**Note: App mode is only available for Chromium-based browsers. Firefox will always open in window mode.**

| Mode | Use Case | UI | Behavior |
|------|----------|----| ---------|
| **Window Mode** | General browsing | Full browser (address bar, tabs, bookmarks) | Opens in new window, can have multiple tabs |
| **App Mode** (Chromium only) | Web applications | Minimal UI (no address bar, no tabs) | Dedicated app-like window, single purpose |

**App Mode is perfect for:**
- Gmail, Calendar, Drive (Google Workspace)
- Slack, Teams, Discord (Communication)
- Trello, Notion, Asana (Productivity)
- Jira, Confluence (Atlassian tools)

### Hyprland Integration

When running on Hyprland, brolaunch automatically applies window rules to improve the browsing experience:

- **Workspace assignment** - Launch browsers on specific workspaces
- **Monitor targeting** - Direct windows to specific monitors
- **Custom window rules** - Apply any Hyprland window rule for fine-tuned control

**Hyprland features work automatically when:**
- Running on Hyprland (detected via `XDG_CURRENT_DESKTOP`)
- `hyprctl` command is available
- Profile configuration includes Hyprland options

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

# Force app mode for any URL (Chromium only)
brolaunch --app https://notion.so
# → Opens in app mode regardless of patterns (ignored for Firefox)

# Open in existing window
brolaunch --existing https://github.com/company/repo
# → Opens in newest existing window for matched profile, or launches new if none

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

   # Hyprland-optimized profile example
   Floating:
     user_data_dir: "/home/user/.config/brolaunch/floating"
     patterns:
       - "notion\\.so"
       - "figma\\.com"
     hyprland_workspace: "3"  # Always open on workspace 3
     hyprland_monitor: "DP-1"  # Open on primary monitor
     hyprland_window_rules:
       - "windowrulev2 = float,class:(chromium)"  # Float all windows
       - "windowrulev2 = size 1200 800,class:(chromium)"  # Set default size
       - "windowrulev2 = center,class:(chromium)"  # Center on screen
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
- App mode is only supported for Chromium-based browsers, not Firefox
- Ensure you're using `--app` flag or `app_patterns` in config
- Some websites may not work well in app mode

**Firefox profile issues:**
- For Firefox, `user_data_dir` must point to an existing profile directory (e.g., `~/.mozilla/firefox/xyz.default`)
- Alternatively, use profile name with `-P` flag (profile must exist in Firefox's profile manager)
- To create a new Firefox profile: `firefox -ProfileManager`

**Hyprland window rules not applying:**
- Ensure you're running on Hyprland (check `XDG_CURRENT_DESKTOP=hyprland`)
- Verify `hyprctl` command is available in PATH
- Use `-v` flag to see if Hyprland rules are being applied
- Rules are applied after browser launch, so window may appear briefly before rules take effect

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
