use crate::{
    component::{Button, Card, Loading},
    fetch,
    route::Route,
    utils::set_title,
};
use reqwest::Method;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[derive(Debug, Deserialize, PartialEq, Clone)]
struct DataFetched {
    id: u32,
    title: String,
    date: String,
}

#[function_component]
pub fn ArticlePreview() -> Html {
    set_title("Articles");

    let is_loading = use_state(|| true);
    let articles = use_state(|| Err("error".into()));

    {
        let articles = articles.clone();
        let is_loading = is_loading.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let fetch_articles =
                        fetch::<Vec<DataFetched>>(Method::GET, "/api/article").await;
                    articles.set(fetch_articles);
                    is_loading.set(false);
                });
            },
            (),
        );
    }

    html! {
        if *is_loading {
            <Loading />
        } else {
            <ArticleGrid resp={(*articles).to_owned()} />
        }
    }
}

#[derive(Debug, PartialEq, Properties)]
struct ArticleGridProps {
    resp: Result<Vec<DataFetched>, String>,
}

#[function_component]
fn ArticleGrid(props: &ArticleGridProps) -> Html {
    let navigator = use_navigator().unwrap();

    let goto_article = |id: u32| {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Article { id });
        })
    };

    let articles = match props.resp.to_owned() {
        Err(e) => html! { e },
        Ok(articles) => articles
            .into_iter()
            .map(|i| {
                html! {
                    <Card title={i.title} key={i.id}>
                        {i.date}
                        <Button content="Goto" onclick={goto_article(i.id)} />
                    </Card>
                }
            })
            .collect::<Html>(),
    };

    html! {
        <div class="grid grid-flow-row-dense grid-cols-3 grid-rows-3 p-10 gap-4">
            {articles}
        </div>
    }
}
