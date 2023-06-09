use crate::{component::Card, fetch, utils::set_title};
use reqwest::Method;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Debug, Deserialize, Clone)]
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
            <Card title="Loading...">
                { "正在加载中..." }
            </Card>
        } else {
            {get_articles( (*articles).clone() )}
        }
    }
}

fn get_articles(resp: Result<Vec<DataFetched>, String>) -> Html {
    let articles = match resp {
        Err(e) => html! { e },
        Ok(articles) => articles
            .into_iter()
            .map(|i| {
                html! {
                    <Card title={i.title} key={i.id}>
                        {i.date}
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
