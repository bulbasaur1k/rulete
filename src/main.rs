mod app;
mod matcher;
mod wheel;
mod ui;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
