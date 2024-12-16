use palette::{Hsl, IntoColor, Srgb, RgbHue};
use std::num::ParseIntError;

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
