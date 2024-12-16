use gloo::cookies::Cookie;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string};

pub struct CookieAdapter;

impl CookieAdapter {
    // Сохраняем значения в куки
    pub fn set<T: Serialize>(key: &str, values: &T) {
        if let Ok(json) = to_string(values) {
            Cookie::set(key, &json);
        } else {
            web_sys::console::error_1(&"Ошибка сериализации данных".into());
        }
    }

    // Получаем значения из куки
    pub fn get<T: DeserializeOwned>(key: &str) -> Option<T> {
        Cookie::get(key).and_then(|cookie| match from_str(&cookie) {
            Ok(values) => Some(values),
            Err(_) => {
                web_sys::console::error_1(&"Ошибка десериализации данных".into());
                None
            }
        })
    }
}
