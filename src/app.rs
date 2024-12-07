use yew::prelude::*;
use crate::wheel::ui::{Wheel};


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <Wheel items={vec!["King Artur", "bulbasaur", "Rust", "Four", "Five", "Six", "Seven", "Ustalovich"].into_iter().map(String::from).collect::<Vec<String>>()} />
        </main>
    }
}
