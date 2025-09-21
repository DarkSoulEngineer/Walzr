use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;
use wallust::colors::Colors;
use wallust::colorspaces::rgb::myrgb_to_hex;

/// Apply GlazeWM colors using a full Colors struct
pub fn apply_glazewm_colors(colors: &Colors) -> Result<(), Box<dyn std::error::Error>> {
    // Choose which Colors fields map to focused / other windows
    let focused_hex = myrgb_to_hex(&colors.color5);
    let other_hex = myrgb_to_hex(&colors.color0);

    let exe_path = r"C:\Program Files\Glzr.io\GlazeWM\glazewm.exe";
    if !Path::new(exe_path).exists() {
        println!("[E] GlazeWM is not installed.");
        return Ok(());
    }
    println!("[I] GlazeWM is installed.");

    let config_path = dirs::home_dir()
        .map(|home| home.join(".glzr/glazewm/config.yaml"))
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;

    if !config_path.exists() {
        eprintln!("[E] Config file not found: {}", config_path.display());
        return Ok(());
    }

    // Backup config
    let mut backup_path = config_path.clone();
    backup_path.set_file_name("bak.config.yaml");
    fs::copy(&config_path, &backup_path)?;
    println!("[I] Backup created: {}", backup_path.display());

    // Update colors
    update_window_colors(&config_path, &focused_hex, &other_hex)?;

    // Reload GlazeWM config
    reload_glazewm_config()?;

    Ok(())
}

fn update_window_colors<P: AsRef<Path>>(
    path: P,
    focused_color: &str,
    other_color: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let yaml_str = fs::read_to_string(&path)?;
    let mut lines: Vec<String> = Vec::new();

    let mut in_focused = false;
    let mut in_other = false;

    for line in yaml_str.lines() {
        let mut new_line = line.to_string();

        if line.contains("focused_window:") {
            in_focused = true;
            in_other = false;
        } else if line.contains("other_windows:") {
            in_other = true;
            in_focused = false;
        }

        if in_focused && line.trim_start().starts_with("color:") {
            let indent = " ".repeat(line.len() - line.trim_start().len());
            new_line = format!("{}color: '{}'", indent, focused_color);
            in_focused = false;
        } else if in_other && line.trim_start().starts_with("color:") {
            let indent = " ".repeat(line.len() - line.trim_start().len());
            new_line = format!("{}color: '{}'", indent, other_color);
            in_other = false;
        }

        lines.push(new_line);
    }

    fs::write(path, lines.join("\n"))?;
    Ok(())
}

fn reload_glazewm_config() -> io::Result<()> {
    let status = Command::new("glazewm")
        .args(&["command", "wm-reload-config"])
        .status()?;

    if status.success() {
        println!("[I] GlazeWM configuration reloaded successfully.");
    } else {
        eprintln!("[E] Failed to reload GlazeWM configuration.");
    }
    Ok(())
}
