use crate::{
    component::{ArticlePreview, Container, Home, NotFound},
    route::Route,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(route: Route) -> Html {
    html! {
        <Container>
        {
            match route {
                Route::Home => html!{<Home />},
                Route::NotFound => html!{<NotFound/>},
                Route::ArticlePreview => html!{<ArticlePreview/>}
            }
        }
        </Container>
    }
}
