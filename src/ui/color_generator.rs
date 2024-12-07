use palette::{Hsl, IntoColor, Srgb, RgbHue};
use std::num::ParseIntError;

/// Функция для парсинга HEX-строки в RGB.
fn hex_to_rgb(hex: &str) -> Result<Srgb<f32>, ParseIntError> {
    let r = u8::from_str_radix(&hex[1..3], 16)?;
    let g = u8::from_str_radix(&hex[3..5], 16)?;
    let b = u8::from_str_radix(&hex[5..7], 16)?;
    Ok(Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0))
}

pub fn generate_neumorphism_colors(count: usize) -> Vec<String> {
    let mut unique_colors = Vec::new();
    let step = 360.0 / count as f32; // Угловое смещение по кругу
    let saturation = 0.4; // Низкая насыщенность для приглушённых оттенков
    let lightness = 0.80; // Высокая светлота для мягкого эффекта

    for i in 0..count {
        let hue = (i as f32 * step) % 360.0; // Распределяем hue равномерно
        let new_color = Hsl::new(RgbHue::from_degrees(hue), saturation, lightness);
        let rgb: Srgb = new_color.into_color();
        unique_colors.push(format!(
            "#{:02X}{:02X}{:02X}",
            (rgb.red * 255.0) as u8,
            (rgb.green * 255.0) as u8,
            (rgb.blue * 255.0) as u8
        ));
    }

    unique_colors
}
