use crate::ui::color_generator::get_unique_colors;
use gloo::timers::callback::Timeout;
use rand::Rng;
use yew::prelude::{function_component, html, use_state, Callback, Html, MouseEvent, Properties};

//#FFC700 yellow
//#4AC99B green
//#70B6F6 blue
#[derive(Properties, Clone, PartialEq)]
pub struct WheelProps {
    pub items: Vec<String>,
}
const CIRCLE: f64 = 360.0;
const FACTOR: f64 = 5.0;
const SPINS: u32 = 5;
#[function_component(Wheel)]
pub fn wheel(props: &WheelProps) -> Html {
    let _name_counts = props.items.len();
    let sector_angle = CIRCLE / _name_counts as f64;

    // Используем UseStateHandle для хранения угла и флага вращения
    let angle = use_state(|| 0.0);
    let is_spinning = use_state(|| false);
    let spin_duration = *angle * FACTOR;
    let colors = get_unique_colors(_name_counts);

    fn calculate_text_position(angle: f64) -> (f64, f64) {
        let x = 100.0 + 80.0 * angle.to_radians().cos();
        let y = 100.0 + 80.0 * angle.to_radians().sin();
        (x, y)
    }

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
                Timeout::new(spin_duration as u32, {
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
            <svg class="wheel-svg" viewBox="0 0 200 200" width="300" height="300"
                style={format!(
                    "transform: rotate({}deg); transition: transform {}ms cubic-bezier(0.42, 0, 0.58, 1);",
                    *angle,  // Текущий угол вращения
                    spin_duration// Время вращения
                )}>
                <circle cx="100" cy="100" r="95" fill="white" />
                {
                   props.items.iter().enumerate().map(|(i, item)| {
                        let start_angle = i as f64 * sector_angle;
                        let end_angle = start_angle + sector_angle;
                        let (x, y) = calculate_text_position(start_angle + sector_angle / 2.0);
                        html! {
                            <>
                                <path class="wheel-segment"
                                    d={generate_sector_path(start_angle, end_angle)}
                                    fill={colors.get(i).unwrap_or(&"#F45725".to_string()).to_string()}
                                />
                                <text class="wheel-text" x={x.to_string()} y={y.to_string()} text-anchor="middle" dominant-baseline="middle" font-size="12">
                                    { item }
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
