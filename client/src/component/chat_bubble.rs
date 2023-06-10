use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[function_component]
pub fn ChatBubble(props: &Props) -> Html {
    let to_bubble = |message| {
        html! {
            <div class="chat chat-start text-lg font-semibold py-3">
              <div class="chat-image avatar">
                <div class="w-12 rounded-full">
                  <img src="/images/avatar.jpg" />
                </div>
              </div>
              <div class="chat-bubble transition hover:-translate-y-3 hover:translate-x-8 hover:scale-110">{message}</div>
            </div>
        }
    };

    props.children.iter().map(to_bubble).collect::<Html>()
}
