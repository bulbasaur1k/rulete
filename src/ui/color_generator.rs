use palette::{Hsl, IntoColor, Srgb, RgbHue};
use std::num::ParseIntError;

/// Функция для парсинга HEX-строки в RGB.
fn hex_to_rgb(hex: &str) -> Result<Srgb<f32>, ParseIntError> {
    let r = u8::from_str_radix(&hex[1..3], 16)?;
    let g = u8::from_str_radix(&hex[3..5], 16)?;
    let b = u8::from_str_radix(&hex[5..7], 16)?;
    Ok(Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0))
}

/// Генерирует массив гармоничных пастельных цветов.
fn generate_pastel_colors(base_colors: Vec<&str>, count: usize) -> Vec<String> {
    let mut unique_colors = Vec::new();

    let step = 360.0 / count as f32; // Угловое смещение по кругу
    let mut color_count = 0; // Для подсчета сгенерированных цветов

    for hex in base_colors.iter() {
        if let Ok(rgb) = hex_to_rgb(hex) {
            // Конвертация в HSL
            let hsl: Hsl = rgb.into_color();
            let mut hue = hsl.hue.into_degrees();

            // Настройки для пастельных цветов
            let saturation = 0.6;  // Уменьшаем насыщенность для пастельных цветов
            let lightness = 0.8;   // Увеличиваем светлоту для более мягких оттенков

            // Генерация новых цветов на основе смещения
            for _ in 0..(count / base_colors.len()) {
                if color_count >= count {
                    break;
                }
                hue = (hue + step) % 360.0; // Смещение hue

                // Преобразуем hue в RgbHue и создаем новый пастельный цвет
                let new_color = Hsl::new(RgbHue::from_degrees(hue), saturation, lightness);
                let rgb: Srgb = new_color.into_color();
                unique_colors.push(format!(
                    "#{:02X}{:02X}{:02X}",
                    (rgb.red * 255.0) as u8,
                    (rgb.green * 255.0) as u8,
                    (rgb.blue * 255.0) as u8
                ));
                color_count += 1;
            }
        }
    }

    // Ограничиваем результат до заданного количества
    unique_colors.truncate(count);
    unique_colors
}
pub fn get_unique_colors(count: usize) -> Vec<String> {
    let base_colors = vec!["#FFC700", "#4AC99B", "#70B6F6"];
    let len = base_colors.len();
    if count <= base_colors.len() {
        return base_colors.into_iter().map(String::from).collect::<Vec<String>>();
    }
    let mut combined = base_colors.clone().into_iter().map(String::from).collect::<Vec<String>>(); // Clone vec1 if you need to preserve it
    combined.extend(generate_pastel_colors(base_colors, count - len));
    combined
}