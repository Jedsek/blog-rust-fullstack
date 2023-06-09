use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Routable, derive_more::Display)]
pub enum Route {
    #[at("/")]
    #[display(fmt = "Home")]
    Home,

    #[not_found]
    #[at("/404")]
    #[display(fmt = "404")]
    NotFound,

    #[at("/article")]
    #[display(fmt = "Articles")]
    ArticlePreview,
}
