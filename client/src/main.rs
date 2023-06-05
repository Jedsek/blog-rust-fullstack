use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    html! {
        <>
            <nav>
                <a href="#" class="px-100">
                    <span>{ "Blog" }</span>
                </a>
                <div>
                    <a href="#" class="text-red">{ "About" }</a>
                </div>
            </nav>
            <h1>{ "Hello World!" }</h1>
            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
                <span class="hover:translate-x-10">    { "Press me!" }    </span>
            </button>

        </>
    }
}
