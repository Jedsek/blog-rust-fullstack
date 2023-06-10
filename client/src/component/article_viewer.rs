use crate::utils::to_html;
use crate::{component::Loading, fetch, utils::set_title};
use pulldown_cmark::html;
use pulldown_cmark::{Options, Parser};
use reqwest::Method;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct ArticleViewerProps {
    pub id: u32,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
struct DataFetched {
    title: String,
    date: String,
    content: String,
}

#[function_component]
pub fn ArticleViewer(props: &ArticleViewerProps) -> Html {
    let is_loading = use_state(|| true);
    let article = use_state(|| Err("".into()));

    {
        let is_loading = is_loading.clone();
        let article = article.clone();
        let id = props.id;
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let fetch_article =
                        fetch::<DataFetched>(Method::GET, &format!("/api/article/{}", id)).await;
                    article.set(fetch_article);
                    is_loading.set(false);
                })
            },
            (),
        );
    }

    html! {
        if *is_loading {
            <Loading />
        } else {
            <Markdown resp={(*article).to_owned()}/>
        }
    }
}

#[derive(Debug, PartialEq, Properties)]
struct MarkdownProps {
    resp: Result<DataFetched, String>,
}

#[function_component]
fn Markdown(props: &MarkdownProps) -> Html {
    let articles = match &props.resp {
        Err(e) => html! { e },
        Ok(article) => {
            set_title(&article.title);
            html! {
                {convert_markdown_to_html(article)}
            }
        }
    };
    html! {
        {articles}
    }
}

fn convert_markdown_to_html(article: &DataFetched) -> Html {
    let content = &article.content;

    let enabled_options = Options::all();
    let parser = Parser::new_ext(content, enabled_options);

    let mut html = String::new();
    html::push_html(&mut html, parser);

    to_html(AttrValue::from(html))
}
