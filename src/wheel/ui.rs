use yew::prelude::*;
use rand::Rng;
use gloo::timers::callback::Timeout;

#[derive(Properties, Clone, PartialEq)]
pub struct WheelProps {
    pub items: Vec<String>,
    pub spin_duration: u64, // в миллисекундах
}

#[function_component(Wheel)]
pub fn wheel(props: &WheelProps) -> Html {
    let sector_angle = 360.0 / props.items.len() as f64;

    // Используем UseStateHandle для хранения угла и флага вращения
    let angle = use_state(|| 0.0);
    let is_spinning = use_state(|| false);

    // Используем callback с помощью Fn
    let on_spin = {
        let set_is_spinning = is_spinning.clone();
        let set_angle = angle.clone();
        let spin_duration = props.spin_duration;

        let is_spinning_value = *is_spinning; // Store the current value

        // Теперь замыкание больше не использует move
        Callback::from({
            let set_is_spinning = set_is_spinning.clone();
            let set_angle = set_angle.clone();
            move |_: MouseEvent| {
                if is_spinning_value {
                    return; // Если уже вращается, не запускаем снова
                }

                set_is_spinning.set(true); // запускаем вращение
                let end_angle = rand::thread_rng().gen_range(720.0..1440.0); // случайный угол

                // Запускаем таймер для завершения вращения
                Timeout::new(spin_duration as u32, {
                    let set_angle = set_angle.clone();
                    let set_is_spinning = set_is_spinning.clone();
                    move || {
                        set_angle.set(end_angle % 360.0); // обновляем угол вращения
                        set_is_spinning.set(false); // останавливаем вращение
                    }
                })
                    .forget(); // Не забываем забыть таймер
            }
        })
    };

    html! {
        <div>
            <svg viewBox="0 0 200 200" width="300" height="300"
                style={format!(
                    "transform: rotate({}deg); transition: transform {}ms cubic-bezier(0.42, 0, 0.58, 1);",
                    *angle,  // разыменовываем angle
                    if *is_spinning { props.spin_duration } else { 0 }
                )}>
                <circle cx="100" cy="100" r="95" fill="white" stroke="black" />
                {
                    props.items.iter().enumerate().map(|(i, item)| {
                        let start_angle = i as f64 * sector_angle;
                        let end_angle = start_angle + sector_angle;
                        let text_angle = start_angle + sector_angle / 2.0;
                        let (x, y) = (
                            100.0 + 80.0 * text_angle.to_radians().cos(),
                            100.0 + 80.0 * text_angle.to_radians().sin()
                        );

                        html! {
                            <>
                                <path
                                    d={format!(
                                        "M100,100 L{} {} A95,95 0 0,1 {} {} Z",
                                        100.0 + 95.0 * start_angle.to_radians().cos(),
                                        100.0 + 95.0 * start_angle.to_radians().sin(),
                                        100.0 + 95.0 * end_angle.to_radians().cos(),
                                        100.0 + 95.0 * end_angle.to_radians().sin()
                                    )}
                                    fill={format!("hsl({}, 70%, 80%)", i * 360 / props.items.len())} />
                                <text x={x.to_string()} y={y.to_string()} text-anchor="middle" dominant-baseline="middle" font-size="12">
                                    { item }
                                </text>
                            </>
                        }
                    }).collect::<Html>()
                }
            </svg>
            <button onclick={on_spin} disabled={*is_spinning}>{ "Spin the Wheel!" }</button>
        </div>
    }
}
