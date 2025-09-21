use std::path::Path;
use wallpaper::{self, Mode};
use windows::core::Result;

/// Set the desktop wallpaper using the `wallpaper` crate.
/// Keeps the original Windows-specific signature.
pub fn set_wallpaper(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(windows::core::Error::new(
            windows::core::HRESULT(0x80070002u32 as i32), // File not found
            format!("File does not exist: {:?}", path).to_string(),
        ));
    }

    let path_str: &str = path.to_str().ok_or_else(|| {
        windows::core::Error::new(
            windows::core::HRESULT(0x80070057u32 as i32), // Invalid parameter
            "Path contains invalid UTF-8".to_string(),
        )
    })?;

    // Set wallpaper image
    wallpaper::set_from_path(path_str).map_err(|e| {
        windows::core::Error::new(
            windows::core::HRESULT(0x80004005u32 as i32), // E_FAIL
            format!("Failed to set wallpaper: {}", e).to_string(),
        )
    })?;

    // Ensure it fills the screen
    wallpaper::set_mode(Mode::Crop).map_err(|e| {
        windows::core::Error::new(
            windows::core::HRESULT(0x80004005u32 as i32),
            format!("Failed to set wallpaper mode: {}", e).to_string(),
        )
    })?;

    Ok(())
}
