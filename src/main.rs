use clap::{Arg, Command as ClapCommand};
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

mod desktop_dialog;
use desktop_dialog::{DesktopEnvironment, SessionType};

#[derive(Debug, Deserialize)]
struct ProfileConfig {
    user_data_dir: Option<String>,
    app_mode: Option<bool>,
    patterns: Option<Vec<String>>,
    app_patterns: Option<Vec<String>>,
    cli_flags: Option<Vec<String>>,
}

#[derive(Debug)]
struct ProfileMatch {
    profile: String,
    app_mode: bool,
}

#[derive(Debug, Deserialize)]
struct Config {
    chromium_binary: Option<String>,
    default_profile: Option<String>,
    profiles: HashMap<String, ProfileConfig>,
}

fn find_config_file(config_path: Option<&str>) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if let Some(path) = config_path {
        return Ok(PathBuf::from(path));
    }

    if let Some(config_dir) = dirs::config_dir() {
        let brolaunch_config = config_dir.join("brolaunch.yaml");
        if brolaunch_config.exists() {
            return Ok(brolaunch_config);
        }
    }

    let local_config = PathBuf::from("config.yaml");
    if local_config.exists() {
        return Ok(local_config);
    }

    Err("No config file found. Looked for ~/.config/brolaunch.yaml and ./config.yaml".into())
}

fn load_config(path: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}

fn match_profile(url: &str, config: &Config) -> Option<ProfileMatch> {
    for (profile_name, profile_config) in &config.profiles {
        // Check app_patterns first (higher priority)
        if let Some(app_patterns) = &profile_config.app_patterns {
            for pattern in app_patterns {
                if let Ok(re) = Regex::new(pattern) {
                    if re.is_match(url) {
                        return Some(ProfileMatch {
                            profile: profile_name.clone(),
                            app_mode: true,
                        });
                    }
                }
            }
        }
        
        // Check regular patterns (window mode)
        if let Some(patterns) = &profile_config.patterns {
            for pattern in patterns {
                if let Ok(re) = Regex::new(pattern) {
                    if re.is_match(url) {
                        return Some(ProfileMatch {
                            profile: profile_name.clone(),
                            app_mode: profile_config.app_mode.unwrap_or(false),
                        });
                    }
                }
            }
        }
    }
    None
}

fn should_include_flag(flag: &str, session_type: &SessionType) -> bool {
    match session_type {
        SessionType::X11 => {
            // Filter out Wayland-specific flags when running on X11
            !flag.contains("--ozone-platform=wayland") && 
            !flag.contains("--enable-features=UseOzonePlatform")
        }
        SessionType::Wayland => {
            // Include all flags on Wayland
            true
        }
        SessionType::Unknown => {
            // When session type is unknown, include all flags (safer default)
            true
        }
    }
}

fn launch_chromium(binary: &str, profile: &str, url: Option<&str>, config: &Config, verbose: bool, app_mode: bool) {
    let mut cmd = Command::new(binary);
    let mut args = Vec::new();
    
    // Check if profile has custom user_data_dir and cli_flags
    let mut user_data_dir_used = None;
    if let Some(profile_config) = config.profiles.get(profile) {
        if let Some(user_data_dir) = &profile_config.user_data_dir {
            let arg = format!("--user-data-dir={}", user_data_dir);
            cmd.arg(&arg);
            args.push(arg);
            user_data_dir_used = Some(user_data_dir.clone());
        }
        
        // Add custom CLI flags (with session-type filtering)
        if let Some(cli_flags) = &profile_config.cli_flags {
            let session_type = DesktopEnvironment::detect_session_type();
            for flag in cli_flags {
                if should_include_flag(flag, &session_type) {
                    cmd.arg(flag);
                    args.push(flag.clone());
                } else if verbose {
                    println!("🚫 Skipping flag '{}' (not compatible with {:?})", flag, session_type);
                }
            }
        }
    }
    
    let profile_arg = format!("--profile-directory={}", profile);
    cmd.arg(&profile_arg);
    args.push(profile_arg);
    
    if let Some(url) = url {
        if app_mode {
            // App mode: creates a dedicated app-like window
            let app_arg = format!("--app={}", url);
            cmd.arg(&app_arg);
            args.push(app_arg);
        } else {
            // Normal mode: new window with URL
            cmd.arg("--new-window");
            args.push("--new-window".to_string());
            cmd.arg(url);
            args.push(url.to_string());
        }
    } else {
        // No URL provided, just open new window
        cmd.arg("--new-window");
        args.push("--new-window".to_string());
    }
    
    if verbose {
        println!("🔧 Profile: {}", profile);
        if let Some(user_data_dir) = &user_data_dir_used {
            println!("📁 User data directory: {}", user_data_dir);
        } else {
            println!("📁 User data directory: default");
        }
        if app_mode {
            println!("📱 Mode: App window");
        } else {
            println!("🪟 Mode: New browser window");
        }
        println!("🚀 Executing: {} {}", binary, args.join(" "));
    }
    
    let status = cmd.status();

    match status {
        Ok(status) if status.success() => {
            if url.is_some() {
                println!("Chromium launched with profile '{}' and URL", profile);
            } else {
                println!("Chromium launched with profile '{}'", profile);
            }
        },
        Ok(status) => eprintln!("Chromium exited with status: {}", status),
        Err(e) => eprintln!("Failed to launch Chromium: {}", e),
    }
}

fn get_available_profiles(config: &Config) -> Vec<String> {
    config.profiles.keys().cloned().collect()
}


fn main() {
    let matches = ClapCommand::new("brolaunch")
        .version("0.1.0")
        .about("Launch Chromium with profile matching for URLs")
        .arg(
            Arg::new("url_or_profile")
                .help("URL to open or profile name to launch directly (optional if default_profile is set)")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Path to config file")
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose logging")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("app")
                .long("app")
                .help("Launch URL as an app (creates app-like window)")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let config_path = matches.get_one::<String>("config").map(|s| s.as_str());
    let verbose = matches.get_flag("verbose");
    let app_mode = matches.get_flag("app");

    // App mode requires a URL
    if app_mode && matches.get_one::<String>("url_or_profile").is_none() {
        eprintln!("Error: --app flag requires a URL to be provided");
        return;
    }

    let config_file = match find_config_file(config_path) {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error finding config file: {}", e);
            return;
        }
    };

    if verbose {
        println!("📋 Using config file: {}", config_file.display());
    }

    let config = match load_config(&config_file) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load config from {:?}: {}", config_file, e);
            return;
        }
    };

    let chromium_binary = config.chromium_binary.as_deref().unwrap_or("chromium");
    
    // Warn about potential wrapper script issues
    if verbose && chromium_binary == "chromium" {
        // Check if we're likely on a system with wrapper scripts
        if std::path::Path::new("/usr/lib/chromium/chromium").exists() {
            println!("⚠️  Warning: Using 'chromium' wrapper script which may cause duplicate flags");
            println!("   Consider setting chromium_binary: \"/usr/lib/chromium/chromium\" in config");
        }
    }
    
    let available_profiles = get_available_profiles(&config);

    if verbose {
        println!("🌐 Chromium binary: {}", chromium_binary);
        println!("👥 Available profiles: [{}]", available_profiles.join(", "));
        if let Some(default) = &config.default_profile {
            println!("⭐ Default profile: {}", default);
        }
        
        let total_patterns: usize = config.profiles.values()
            .filter_map(|p| p.patterns.as_ref())
            .map(|patterns| patterns.len())
            .sum();
        let total_app_patterns: usize = config.profiles.values()
            .filter_map(|p| p.app_patterns.as_ref())
            .map(|patterns| patterns.len())
            .sum();
        println!("🔍 URL patterns configured: {} regular, {} app patterns", total_patterns, total_app_patterns);
        
        for (profile, profile_config) in &config.profiles {
            let regular_count = profile_config.patterns.as_ref().map_or(0, |p| p.len());
            let app_count = profile_config.app_patterns.as_ref().map_or(0, |p| p.len());
            if regular_count > 0 || app_count > 0 {
                println!("  {}: {} regular, {} app patterns", profile, regular_count, app_count);
            }
        }
        println!();
    }

    // Handle case where no arguments are provided
    if let Some(url_or_profile) = matches.get_one::<String>("url_or_profile") {
        if verbose {
            println!("🔤 Input: {}", url_or_profile);
        }
        
        // Check if the input matches a profile name (case-insensitive)
        let profile_match = available_profiles.iter()
            .find(|&profile| profile.to_lowercase() == url_or_profile.to_lowercase());

        if let Some(profile) = profile_match {
            // Direct profile launch without URL
            if verbose {
                println!("✅ Matched profile name: {}", profile);
            }
            launch_chromium(chromium_binary, profile, None, &config, verbose, app_mode);
        } else if url_or_profile.contains("://") || url_or_profile.contains(".") {
            // Treat as URL - check for regex pattern matches
            if verbose {
                println!("🌐 Treating as URL, checking regex patterns...");
            }
            
            if let Some(profile_match) = match_profile(url_or_profile, &config) {
                let final_app_mode = app_mode || profile_match.app_mode;
                if verbose {
                    println!("✅ URL matched regex pattern for profile: {}", profile_match.profile);
                    if profile_match.app_mode {
                        println!("📱 Pattern configured for app mode");
                    }
                    if app_mode && !profile_match.app_mode {
                        println!("🔧 CLI --app flag overriding pattern default");
                    }
                }
                launch_chromium(chromium_binary, &profile_match.profile, Some(url_or_profile), &config, verbose, final_app_mode);
            } else {
                if verbose {
                    println!("❌ No regex patterns matched, showing profile chooser");
                }
                println!("No profile matched for URL: {}", url_or_profile);
                
                if let Some(selected_profile) = desktop_dialog::show_profile_chooser_with_debug(&available_profiles, verbose) {
                    if verbose {
                        println!("👆 User selected profile: {}", selected_profile);
                    }
                    launch_chromium(chromium_binary, &selected_profile, Some(url_or_profile), &config, verbose, app_mode);
                } else {
                    println!("No profile selected. Exiting.");
                }
            }
        } else {
            // Not a recognized profile or URL format
            if verbose {
                println!("❌ Input doesn't match any profile or URL pattern");
            }
            eprintln!("'{}' is not a recognized profile name or valid URL", url_or_profile);
            eprintln!("Available profiles: {}", available_profiles.join(", "));
        }
    } else {
        // No arguments provided - use default profile if configured
        if verbose {
            println!("🏠 No arguments provided, checking for default profile...");
        }
        
        if let Some(default_profile) = &config.default_profile {
            if verbose {
                println!("✅ Using default profile: {}", default_profile);
            }
            launch_chromium(chromium_binary, default_profile, None, &config, verbose, app_mode);
        } else {
            if verbose {
                println!("❌ No default profile configured");
            }
            eprintln!("No arguments provided and no default_profile configured.");
            eprintln!("Available profiles: {}", available_profiles.join(", "));
            eprintln!("Usage: brolaunch [URL_OR_PROFILE]");
        }
    }
}
