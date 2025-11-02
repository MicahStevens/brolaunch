use std::env;
use std::process::Command;

#[derive(Debug, Clone, PartialEq)]
pub enum DialogType {
    Kdialog,
    Zenity,
    Wofi,
    Fuzzel,
    Hyprpicker,
    Terminal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SessionType {
    Wayland,
    X11,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct DesktopEnvironment {
    pub name: String,
    pub session_type: SessionType,
    pub dialog_type: DialogType,
}

impl DesktopEnvironment {
    pub fn detect() -> Self {
        let session_type = Self::detect_session_type();
        let desktop_name = Self::detect_desktop_name();
        let dialog_type = Self::detect_best_dialog(&desktop_name, &session_type);
        
        DesktopEnvironment {
            name: desktop_name,
            session_type,
            dialog_type,
        }
    }
    
    pub fn detect_session_type() -> SessionType {
        if let Ok(session_type) = env::var("XDG_SESSION_TYPE") {
            match session_type.to_lowercase().as_str() {
                "wayland" => return SessionType::Wayland,
                "x11" => return SessionType::X11,
                _ => {}
            }
        }
        
        // Fallback detection via display environment variables
        let wayland_display = env::var("WAYLAND_DISPLAY").is_ok();
        let x11_display = env::var("DISPLAY").is_ok();
        
        match (x11_display, wayland_display) {
            (false, true) => SessionType::Wayland,
            (true, false) => SessionType::X11,
            (true, true) => SessionType::Wayland, // Prefer Wayland if both are available
            (false, false) => SessionType::Unknown,
        }
    }
    
    fn detect_desktop_name() -> String {
        // Try XDG_CURRENT_DESKTOP first (most reliable)
        if let Ok(desktop) = env::var("XDG_CURRENT_DESKTOP") {
            return desktop.to_lowercase();
        }
        
        // Fallback to XDG_SESSION_DESKTOP
        if let Ok(desktop) = env::var("XDG_SESSION_DESKTOP") {
            return desktop.to_lowercase();
        }
        
        // Fallback to DESKTOP_SESSION
        if let Ok(desktop) = env::var("DESKTOP_SESSION") {
            return desktop.to_lowercase();
        }
        
        // Last resort: parse XDG_DATA_DIRS
        if let Ok(xdg_dirs) = env::var("XDG_DATA_DIRS") {
            for dir in xdg_dirs.split(':') {
                if dir.contains("kde") {
                    return "kde".to_string();
                } else if dir.contains("gnome") {
                    return "gnome".to_string();
                } else if dir.contains("xfce") {
                    return "xfce".to_string();
                }
            }
        }
        
        "unknown".to_string()
    }
    
    fn detect_best_dialog(desktop_name: &str, session_type: &SessionType) -> DialogType {
        // Check desktop-specific preferences first
        match desktop_name {
            name if name.contains("kde") || name.contains("plasma") => {
                if Self::command_exists("kdialog") {
                    return DialogType::Kdialog;
                } else if Self::command_exists("zenity") {
                    return DialogType::Zenity;
                }
            }
            name if name.contains("gnome") || name.contains("unity") || name.contains("cinnamon") => {
                if Self::command_exists("zenity") {
                    return DialogType::Zenity;
                } else if Self::command_exists("kdialog") {
                    return DialogType::Kdialog;
                }
            }
            name if name.contains("hyprland") => {
                if *session_type == SessionType::Wayland {
                    if Self::command_exists("hyprpicker") {
                        return DialogType::Hyprpicker;
                    } else if Self::command_exists("wofi") {
                        return DialogType::Wofi;
                    } else if Self::command_exists("fuzzel") {
                        return DialogType::Fuzzel;
                    } else if Self::command_exists("zenity") {
                        return DialogType::Zenity; // via XWayland
                    }
                } else if Self::command_exists("zenity") {
                    return DialogType::Zenity;
                } else if Self::command_exists("kdialog") {
                    return DialogType::Kdialog;
                }
            }
            name if name.contains("sway") || name.contains("river") => {
                if *session_type == SessionType::Wayland {
                    if Self::command_exists("wofi") {
                        return DialogType::Wofi;
                    } else if Self::command_exists("fuzzel") {
                        return DialogType::Fuzzel;
                    } else if Self::command_exists("zenity") {
                        return DialogType::Zenity; // via XWayland
                    }
                } else if Self::command_exists("zenity") {
                    return DialogType::Zenity;
                } else if Self::command_exists("kdialog") {
                    return DialogType::Kdialog;
                }
            }
            _ => {}
        }
        
        // Generic detection based on available tools and session type
        if *session_type == SessionType::Wayland {
            if Self::command_exists("wofi") {
                return DialogType::Wofi;
            } else if Self::command_exists("fuzzel") {
                return DialogType::Fuzzel;
            }
        }
        
        // Universal fallbacks
        if Self::command_exists("zenity") {
            DialogType::Zenity
        } else if Self::command_exists("kdialog") {
            DialogType::Kdialog
        } else {
            // Final fallback to terminal-based selection
            DialogType::Terminal
        }
    }
    
    fn command_exists(command: &str) -> bool {
        Command::new("which")
            .arg(command)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

pub fn show_profile_chooser_with_debug(profiles: &[String], verbose: bool) -> Option<String> {
    let desktop_env = DesktopEnvironment::detect();
    
    if verbose {
        println!("🖥️  Desktop: {} ({:?})", desktop_env.name, desktop_env.session_type);
        println!("💬 Dialog system: {:?}", desktop_env.dialog_type);
    }
    
    match desktop_env.dialog_type {
        DialogType::Kdialog => show_kdialog_chooser(profiles),
        DialogType::Zenity => show_zenity_chooser(profiles),
        DialogType::Wofi => show_wofi_chooser(profiles),
        DialogType::Fuzzel => show_fuzzel_chooser(profiles),
        DialogType::Hyprpicker => show_hyprpicker_chooser(profiles),
        DialogType::Terminal => show_terminal_chooser(profiles),
    }
}

fn show_kdialog_chooser(profiles: &[String]) -> Option<String> {
    let mut cmd = Command::new("kdialog");
    cmd.arg("--title").arg("Select Profile");
    cmd.arg("--menu").arg("Choose a browser profile:");
    
    // Add numbered options for kdialog
    for (i, profile) in profiles.iter().enumerate() {
        cmd.arg(format!("{}", i + 1));
        cmd.arg(profile);
    }
    
    match cmd.output() {
        Ok(output) if output.status.success() => {
            let selection = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if let Ok(index) = selection.parse::<usize>() {
                if index > 0 && index <= profiles.len() {
                    return Some(profiles[index - 1].clone());
                }
            }
            None
        }
        _ => None,
    }
}

fn show_zenity_chooser(profiles: &[String]) -> Option<String> {
    let mut cmd = Command::new("zenity");
    cmd.arg("--list");
    cmd.arg("--title=Select Profile");
    cmd.arg("--text=Choose a browser profile:");
    cmd.arg("--column=Profile");
    
    for profile in profiles {
        cmd.arg(profile);
    }
    
    match cmd.output() {
        Ok(output) if output.status.success() => {
            let selection = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !selection.is_empty() && profiles.contains(&selection) {
                Some(selection)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn show_wofi_chooser(profiles: &[String]) -> Option<String> {
    let mut cmd = Command::new("wofi");
    cmd.arg("--dmenu");
    cmd.arg("--prompt=Select Profile:");
    cmd.arg("--width=300");
    cmd.arg("--height=200");
    cmd.arg("--location=center");
    
    let input = profiles.join("\n");
    
    match cmd.stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
    {
        Ok(mut child) => {
            if let Some(stdin) = child.stdin.take() {
                use std::io::Write;
                let _ = std::thread::spawn(move || {
                    let mut stdin = stdin;
                    let _ = stdin.write_all(input.as_bytes());
                });
            }
            
            match child.wait_with_output() {
                Ok(output) if output.status.success() => {
                    let selection = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !selection.is_empty() && profiles.contains(&selection) {
                        Some(selection)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}

fn show_fuzzel_chooser(profiles: &[String]) -> Option<String> {
    let mut cmd = Command::new("fuzzel");
    cmd.arg("--dmenu");
    cmd.arg("--prompt=Profile: ");
    
    let input = profiles.join("\n");
    
    match cmd.stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
    {
        Ok(mut child) => {
            if let Some(stdin) = child.stdin.take() {
                use std::io::Write;
                let _ = std::thread::spawn(move || {
                    let mut stdin = stdin;
                    let _ = stdin.write_all(input.as_bytes());
                });
            }
            
            match child.wait_with_output() {
                Ok(output) if output.status.success() => {
                    let selection = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !selection.is_empty() && profiles.contains(&selection) {
                        Some(selection)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}

fn show_hyprpicker_chooser(profiles: &[String]) -> Option<String> {
    // Hyprpicker is primarily for color picking, but we can use it creatively
    // For now, fall back to terminal chooser but with Hyprland-specific messaging
    println!("\n🪟 Hyprland Profile Selector");
    println!("Available profiles:");
    for (i, profile) in profiles.iter().enumerate() {
        println!("  {}: {}", i + 1, profile);
    }

    print!("Select profile (1-{}): ", profiles.len());
    use std::io::{self, Write};
    let _ = io::stdout().flush();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            if let Ok(index) = input.trim().parse::<usize>() {
                if index > 0 && index <= profiles.len() {
                    return Some(profiles[index - 1].clone());
                }
            }
            None
        }
        _ => None,
    }
}

fn show_terminal_chooser(profiles: &[String]) -> Option<String> {
    println!("\nAvailable profiles:");
    for (i, profile) in profiles.iter().enumerate() {
        println!("  {}: {}", i + 1, profile);
    }

    print!("Select profile (1-{}): ", profiles.len());
    use std::io::{self, Write};
    let _ = io::stdout().flush();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            if let Ok(index) = input.trim().parse::<usize>() {
                if index > 0 && index <= profiles.len() {
                    return Some(profiles[index - 1].clone());
                }
            }
            None
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_session_type_detection() {
        // Test will depend on the current environment
        let session_type = DesktopEnvironment::detect_session_type();
        assert!(matches!(session_type, SessionType::Wayland | SessionType::X11 | SessionType::Unknown));
    }
    
    #[test]
    fn test_desktop_detection() {
        let desktop_env = DesktopEnvironment::detect();
        assert!(!desktop_env.name.is_empty());
    }
    
    #[test]
    fn test_command_exists() {
        // Test with a command that should exist on most systems
        assert!(DesktopEnvironment::command_exists("echo"));
        // Test with a command that likely doesn't exist
        assert!(!DesktopEnvironment::command_exists("nonexistent_command_12345"));
    }
}