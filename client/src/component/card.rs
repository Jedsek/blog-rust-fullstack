use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub title: AttrValue,
    pub children: Children,
}

#[function_component]
pub fn Card(props: &Props) -> Html {
    html! {
        <article class="p-7 hover:bg-slate-200">
            <header class="text-3xl font-bold">
                <h3>{ &props.title }</h3>
            </header>
            <footer>
                { for props.children.iter() }
            </footer>
        </article>
    }
}
