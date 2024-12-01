mod app;
mod matcher;
mod wheel;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
