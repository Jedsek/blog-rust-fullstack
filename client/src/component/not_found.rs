use crate::component::card::Card;
use yew::prelude::*;

#[function_component]
pub fn NotFound() -> Html {
    use_context::<Callback<String>>()
        .unwrap()
        .emit("404".into());

    html! {
        <Card title={"404-error"}>
            <p>{ "尝试换个地址?" }</p>
        </Card>
    }
}
