use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component]
pub fn ArticleViewer() -> Html {
    // let is_loading = use_state(|| true);
    // let articles = use_state(|| Err("".into()));

    {
        // let _is_loading = is_loading.clone();
        // let articles = articles.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    // let fetch_articles = fetch::<()>(Method::GET, "/api/article");
                })
            },
            (),
        );
    }

    html! {
        "123"
    }
}
