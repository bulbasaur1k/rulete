use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct CircleProps {
    pub name: String
}

#[function_component(Circle)]
pub fn circle(props: &CircleProps) -> Html {
    html! {
        <div>{format!("My: {}", props.name)}</div>
    }
}