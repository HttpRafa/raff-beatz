use app::App;

pub mod app;
pub mod components;
pub mod sites;

fn main() {
    yew::Renderer::<App>::new().render();
}
