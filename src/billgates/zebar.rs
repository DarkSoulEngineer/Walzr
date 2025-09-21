use std::fs;
use std::io;
use std::path::Path;
use wallust::colors::{Colors, Myrgb};

/// Apply a Colors struct directly to CSS variables in styles.css
pub fn apply_zebar_colors(colors: &Colors) -> Result<(), Box<dyn std::error::Error>> {
    let css_path = dirs::home_dir()
        .map(|home| home.join(r"AppData\Roaming\zebar\downloads\echosonusharma.neon@1.0.1\styles.css"))
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;

    if !css_path.exists() {
        return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, format!("CSS file not found: {}", css_path.display()))));
    }

    // Backup CSS
    let backup_path = css_path.with_file_name("styles.bak.css");
    fs::copy(&css_path, &backup_path)?;
    println!("[I] Backup created: {}", backup_path.display());

    // Map Colors -> (Myrgb, CSS variable)
    let theme_colors: Vec<(Myrgb, &str)> = vec![
        (colors.color5, "color-primary"),
        (colors.color5, "color-primary-light"),
        (colors.color2, "color-primary-hover"),
        (colors.color5, "color-primary-glow"),
        (colors.color3, "color-primary-box"),
        (colors.color3, "color-primary-box-strong"),
        (colors.color0, "color-secondary"),
        (colors.color0, "color-secondary-transparent"),
        (colors.color7, "color-white"),
        (colors.color7, "color-white-strong"),
        (colors.color1, "color-alert"),
        (colors.color1, "color-alert-glow"),
        (colors.color5, "color-battery-full"),
        (colors.color5, "color-battery-full-glow"),
    ];


    // Convert Myrgb -> "r g b"
    let updates: Vec<(&str, String)> = theme_colors
        .iter()
        .map(|(color, var_name)| (*var_name, myrgb_to_rgb_string(color)))
        .collect();

    // Update CSS variables
    update_css_colors(&css_path, &updates)?;

    println!("[I] CSS colors updated successfully.");
    Ok(())
}

/// Convert Myrgb -> "r g b" string
fn myrgb_to_rgb_string(color: &Myrgb) -> String {
    format!(
        "{} {} {}",
        (color.0.red * 255.0).round() as u8,
        (color.0.green * 255.0).round() as u8,
        (color.0.blue * 255.0).round() as u8
    )
}

/// Update CSS variables in styles.css while preserving alpha
fn update_css_colors<P: AsRef<Path>>(
    path: P,
    updates: &[(&str, String)],
) -> Result<(), Box<dyn std::error::Error>> {
    let css_str = fs::read_to_string(&path)?;
    let mut lines: Vec<String> = Vec::new();

    for line in css_str.lines() {
        let mut new_line = line.to_string();
        let trimmed = line.trim_start();

        for (var, rgb_str) in updates {
            if trimmed.starts_with(&format!("--{}:", var)) {
                let indent = " ".repeat(line.len() - trimmed.len());
                if let Some((_, value_part)) = trimmed.split_once(':') {
                    let value = value_part.trim_end_matches(';').trim();
                    if let Some((_, alpha)) = value.split_once('/') {
                        let alpha_trimmed = alpha.trim().trim_end_matches(')');
                        new_line = format!("{}--{}: rgb({} / {});", indent, var, rgb_str, alpha_trimmed);
                    } else {
                        new_line = format!("{}--{}: rgb({});", indent, var, rgb_str);
                    }
                }
                break;
            }
        }

        lines.push(new_line);
    }

    fs::write(path, lines.join("\n"))?;
    Ok(())
}