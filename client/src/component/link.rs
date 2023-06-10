use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub url: AttrValue,
    pub display: AttrValue,
}

#[function_component]
pub fn Link(props: &Props) -> Html {
    html! {
        <div class="tooltip tooltip-bottom" data-tip={&props.url}>
            <a class="link link-accent" target="_blank" href={&props.url}> {&props.display} </a>
        </div>
    }
}
