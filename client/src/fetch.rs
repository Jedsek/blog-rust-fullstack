use reqwest::{Error, Method, RequestBuilder, Response};
use serde::Serialize;

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.3";
const SERVER_BASE_URL: &str = "http://127.0.0.1:8080";

#[rustfmt::skip]
pub async fn fetch<W>(method: Method, api_url: &str) -> Result<W, String>
where
    W: for<'de> serde::Deserialize<'de>,
{
    let req = build_request(method, api_url);
    let resp = req.send().await;
    handle_response(resp).await
}

#[rustfmt::skip]
#[allow(dead_code)]
pub async fn fetch_with_json<T, W>(method: Method, api_url: &str, json: T) -> Result<W, String>
where
    T: Serialize,
    W: for<'de> serde::Deserialize<'de>,
{
    let req = build_request(method, api_url);
    let resp = req.json(&json).send().await;
    handle_response(resp).await
}

fn build_request(method: Method, api_url: &str) -> RequestBuilder {
    fn relative_url(path: &str) -> String {
        let base = String::from(SERVER_BASE_URL);
        base + path
    }

    let api_url = relative_url(api_url);
    reqwest::Client::new()
        .request(method, api_url)
        .header("User-Agent", USER_AGENT)
}

#[rustfmt::skip]
async fn handle_response<W>(resp: Result<Response, Error>) -> Result<W, String>
where
    W: for<'de> serde::Deserialize<'de>,
{
    match resp {
        Ok(resp) if resp.status().is_success() => 
            resp.json::<W>().await.map_err(|e| format!("无法将响应解析为Json格式: {e}")),
        Ok(resp) => 
            Err(format!(
                "({}) : {}",
                resp.status(),
                resp.text().await.unwrap(),
            )),
        Err(e) => 
            Err(format!("无法发送请求: {e}")),
    }
}
