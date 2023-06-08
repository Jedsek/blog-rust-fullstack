mod app;
mod component;
mod route;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
