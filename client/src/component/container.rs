use crate::{component::NavBar, route::Route};
use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[function_component]
pub fn Container(props: &Props) -> Html {
    html! {
        <>
            // <NavBar/>

            <NavBar
                title="Blog"
                routes={vec![
                    Route::Home,
                    Route::ArticlePreview,
                    Route::NotFound
                ]}
            />

            <progress class="progress"></progress>


            { for props.children.iter() }


        </>
    }
}
