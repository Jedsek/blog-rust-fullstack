use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

// #[rustfmt::skip]
#[function_component]
pub fn ChatBubble(props: &Props) -> Html {
    let to_bubble = |message| {
        html! {
            <div class="chat chat-start text-xl font-semibold py-3">
              <div class="chat-bubble">{message}</div>
            </div>
        }
    };

    props.children.iter().map(to_bubble).collect::<Html>()
}
