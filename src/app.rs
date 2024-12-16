use yew::prelude::*;
use crate::wheel::ui::{Wheel};


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <Wheel items={vec!["Андрей", "Артур", "Алексей", "Александр Ш", "Александр Б", "Евгений П", "Евгений Н", "Вадим", "Константин", "Петр", "Рустем", "Сергей", "Даниил"].into_iter().map(String::from).collect::<Vec<String>>()} />
        </main>
    }
}
