use crate::ui::color_generator::generate_neumorphism_colors;
use gloo::timers::callback::Timeout;
use gloo_console::log;
use gloo_storage::{LocalStorage, Storage};
use rand::Rng;
use yew::prelude::{function_component, html, use_state, Callback, Html, MouseEvent, Properties};
use serde::{Deserialize, Serialize};
use yew::{use_effect, use_effect_with, UseStateHandle};

//#FFC700 yellow
//#4AC99B green
//#70B6F6 blue
#[derive(Properties, Clone, PartialEq)]
pub struct WheelProps {
    pub items: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct KeyValue {
    key: String,
    value: f64,
}

const CIRCLE: f64 = 360.0;
const SPINS: u32 = 5;
const STORAGE_KEY: &str = "vba_team";
const SPIN_DURATION: u32 = 8000;

fn update_item_by_key(key: &str, new_value: f64, items: &UseStateHandle<Vec<KeyValue>>) {
    // Clone the current state into an owned vector
    let mut updated_items = (**items).clone();

    // Find the item by key and update it
    if let Some(item) = updated_items.iter_mut().find(|item| item.key == key) {
        item.value = new_value;
    }

    // Update LocalStorage
    LocalStorage::set(STORAGE_KEY, &updated_items).expect("Failed to update LocalStorage");

    // Update the state
    items.set(updated_items);
}

fn log_items_on_change(items: &UseStateHandle<Vec<KeyValue>>) {
    use_effect_with(
        items.clone(),
        {
            let items = items.clone();
            move |_| {
                // Логируем текущее состояние items в консоль
                log!(format!("Items updated: {:?}", *items));

                // Возвращаем функцию очистки, если требуется (в данном случае нет)
                || ()
            }
        }
    );
}

#[function_component(Wheel)]
pub fn wheel(props: &WheelProps) -> Html {
    let _name_counts = props.items.len();
    let sector_angle = CIRCLE / _name_counts as f64;
    let items = use_state(|| vec![] as Vec<KeyValue>);
    log_items_on_change(&items);
    use_effect_with((), {
        let items_clone = props.items.clone(); // Clone props.items
        let items = items.clone(); // Clone items state handle for use in the closure

        move |_| {
            let initial_items: Vec<KeyValue> = LocalStorage::get(STORAGE_KEY).unwrap_or_else(|_| vec![]);

            if initial_items.is_empty() {
                let res: Vec<KeyValue> = items_clone
                    .iter()
                    .map(|s| KeyValue {
                        key: s.to_string(),
                        value: 0.1f64,
                    })
                    .collect();
                items.set(res);
            } else {
                items.set(initial_items);
            }

            || {} // Cleanup callback (no-op in this case)
        }
    });


    // Используем UseStateHandle для хранения угла и флага вращения
    let angle = use_state(|| 0.0);
    let is_spinning = use_state(|| false);
    let colors = generate_neumorphism_colors(_name_counts);

    // Функция для генерации пути сектора колеса
    fn generate_sector_path(start_angle: f64, end_angle: f64) -> String {
        format!(
            "M100,100 L{} {} A95,95 0 0,1 {} {} Z",
            100.0 + 95.0 * start_angle.to_radians().cos(),
            100.0 + 95.0 * start_angle.to_radians().sin(),
            100.0 + 95.0 * end_angle.to_radians().cos(),
            100.0 + 95.0 * end_angle.to_radians().sin()
        )
    }

    // Логика для вращения колеса
    let on_spin = {
        let set_is_spinning = is_spinning.clone();
        let set_angle = angle.clone();

        Callback::from({
            let set_is_spinning = set_is_spinning.clone();
            let set_angle = set_angle.clone();
            move |_: MouseEvent| {
                if *set_is_spinning {
                    return; // Если уже вращается, не запускаем снова
                }

                set_is_spinning.set(true); // Запускаем вращение

                // Генерируем случайный конечный угол (многократное вращение)
                let mut rng = rand::thread_rng();
                let size_section = CIRCLE / _name_counts as f64;
                let center_section = size_section / 2.0;
                let rand_name = rng.gen_range(0.._name_counts) as f64 * size_section;
                let end_angle = SPINS as f64 * CIRCLE + rand_name + center_section;

                // Обновляем угол вращения
                set_angle.set(*set_angle + end_angle);

                // Устанавливаем таймер для завершения вращения
                Timeout::new(SPIN_DURATION, {
                    let set_is_spinning = set_is_spinning.clone();
                    move || {
                        set_is_spinning.set(false); // Останавливаем вращение
                    }
                })
                .forget();
            }
        })
    };

    html! {
        <div class="wheel-container">
            <div class="pointer"></div>
            <div class="wheel-shadow"></div>
            <svg class="wheel-svg" viewBox="0 0 200 200" width="300" height="300"
                style={format!(
                    "transform: rotate({}deg); transition: transform {}ms cubic-bezier(0.42, 0, 0.58, 1);",
                    *angle,  // Текущий угол вращения
                    SPIN_DURATION// Время вращения
                )}>
                <circle cx="100" cy="100" r="95" fill="white" />
                {
                   items.iter().enumerate().map(|(i, item)| {
                        let start_angle = i as f64 * sector_angle;
                        let end_angle = start_angle + sector_angle;
                        html! {
                            <>
                                <path class="wheel-segmen" 
                                    d={generate_sector_path(start_angle, end_angle)}
                                    fill={colors[i].to_string()} />
                                <text class="wheel-text"
                                    text-anchor="middle"
                                    dominant-baseline="middle"
                                    transform={format!(
                                        "translate({:.2},{:.2}) rotate({:.2})",
                                        100.0 + 60.0 * ((start_angle + sector_angle / 2.0).to_radians().cos()),
                                        100.0 + 60.0 * ((start_angle + sector_angle / 2.0).to_radians().sin()),
                                        start_angle + sector_angle / 2.0 + 180.0 // Корректируем угол для ориентации "наружу"
                                    )}>
                                    { item.key.clone() }
                                </text>
                            </>
                        }
                    }).collect::<Html>()
                }
            </svg>
            <button class="wheel-button" onclick={on_spin} disabled={*is_spinning}>{ "Spin the Wheel!" }</button>
        </div>
    }
}
