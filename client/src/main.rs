mod app;
mod component;
mod fetch;
mod route;
mod utils;
pub use fetch::fetch;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
