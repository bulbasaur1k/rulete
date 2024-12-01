use yew::prelude::*;
use crate::wheel::ui::{Wheel};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <Wheel items={vec!["One", "Two", "Three", "Four", "Five"].into_iter().map(String::from).collect::<Vec<String>>()} spin_duration={1000} />
        </main>
    }
}
