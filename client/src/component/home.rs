use crate::component::Card;
use yew::prelude::*;

#[function_component]
pub fn Home() -> Html {
    use_context::<Callback<String>>()
        .unwrap()
        .emit("Home".into());

    html! {
        <Card title={"首页"}>
            <p>{ "这是首页" }</p>
        </Card>
    }
}
