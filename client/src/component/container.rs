use crate::component::NavBar;
use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[function_component]
pub fn Container(props: &Props) -> Html {
    let set_title = Callback::from(|title: String| gloo::utils::document().set_title(&title));

    html! {
        <>
            <NavBar/>

            <ContextProvider<Callback<String>> context={set_title}>
                { for  props.children.iter() }
            </ContextProvider<Callback<String>>>
        </>
    }
}
