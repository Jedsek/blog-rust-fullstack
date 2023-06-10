use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub content: AttrValue,
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn Button(props: &Props) -> Html {
    html! {
        <div class="card-actions justify-end">
            <button class="btn normal-case transition ease-in-out hover:translate-y-1 hover:translate-x-1 hover:scale-110" onclick={props.onclick.clone()}>
                {&props.content}
            </button>
        </div>
    }
}
