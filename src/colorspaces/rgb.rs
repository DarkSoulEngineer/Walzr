use palette::Srgb;          // import Srgb
use crate::colorspaces::Myrgb; // import your Myrgb wrapper

/// Convert Srgb to HEX string
fn rgb_to_hex(rgb: Srgb) -> String {
    let r = (rgb.red * 255.0).round() as u8;
    let g = (rgb.green * 255.0).round() as u8;
    let b = (rgb.blue * 255.0).round() as u8;
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

/// Convert your Myrgb wrapper to HEX
pub fn myrgb_to_hex(color: &Myrgb) -> String {
    rgb_to_hex(color.0) // tuple struct, so use .0 to get inner Srgb
}
