use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    html! {
        <>
            <nav>
                <a href="#" class="brand">
                    <span>{ "Blog" }</span>
                </a>
                <div class="menu">
                    <a href="#" class="button icon-puzzle">{ "About" }</a>
                </div>
            </nav>
            <h1>{ "Hello World!" }</h1>
        </>
    }
}
