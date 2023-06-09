use crate::{component::card::Card, utils::set_title};
use yew::prelude::*;

#[function_component]
pub fn NotFound() -> Html {
    set_title("404");

    html! {
        <Card title={"404"}>
            <p>{ "尝试换个地址?" }</p>
        </Card>
    }
}
