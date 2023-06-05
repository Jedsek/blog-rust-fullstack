use crate::{
    error::CustomError,
    model::user::{AccessToekn, Login},
    AppState, USER_AGENT,
};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use cookie::{time::Duration, Cookie};
use once_cell::sync::OnceCell;
use reqwest::ClientBuilder;
use shared::users::GithubUserInfo;

type OauthKey = (String, String);
static GITHUB_OAUTH_KEY: OnceCell<OauthKey> = OnceCell::new();

fn get_github_oauth_key() -> Result<OauthKey, CustomError> {
    let project_root_path = String::from(env!("CARGO_MANIFEST_DIR"));
    let client_id = {
        let file_name = "/oauth_id.txt";
        let path = project_root_path.clone() + file_name;
        std::fs::read_to_string(path)?
    };
    let client_secret = {
        let file_name = "/oauth_secret.txt";
        let path = project_root_path + file_name;
        std::fs::read_to_string(path)?
    };
    Ok((client_id, client_secret))
}

#[post("/login")]
pub async fn github_login(
    Json(login): Json<Login>,
    state: Data<AppState>,
) -> Result<impl Responder, CustomError> {
    let code = login.code;
    let client = ClientBuilder::new().user_agent(USER_AGENT).build()?;
    let pool = &state.pool;

    let (client_id, client_secret) = GITHUB_OAUTH_KEY
        .get_or_try_init(get_github_oauth_key)
        .map(|key| key.to_owned())?;

    let result = client.get(format!("https://github.com/login/oauth/access_token?client_id={client_id}&client_secret={client_secret}&code={code}"))
        .header("Accept", "application/json")
        .send()
        .await;

    // 进行请求, 解析结果, 若无法序列化(解析成功), 则返回错误
    let access_token = match result {
        Ok(resp) => match resp.json::<AccessToekn>().await {
            Ok(access_token) => access_token.access_token,
            Err(_) => {
                return Err(CustomError::AuthFailed(
                    "code 无效(可能过期), 请使用 Github 登陆".into(),
                ))
            }
        },
        Err(e) => {
            return Err(CustomError::InternalError(format!(
                "无法获取 access_token, 请重试: {}",
                e.to_string()
            )))
        }
    };

    let user_info = client
        .get("https://api.github.com/user")
        .bearer_auth(access_token.clone())
        .send()
        .await;

    let user_info = match user_info {
        Ok(resp) => resp.json::<GithubUserInfo>().await.unwrap(),
        Err(_) => {
            return Err(CustomError::InternalError(
                "无法获取 Github 用户信息, 请重试".into(),
            ))
        }
    };

    sqlx::query("insert or replace into users (id, name, avatar_url) values ($1, $2, $3)")
        .bind(user_info.id)
        .bind(&user_info.login)
        .bind(user_info.avatar_url)
        .execute(pool)
        .await?;

    let cookie_access_token = Cookie::build("ACCESS_TOKEN", access_token)
        .path("/")
        .max_age(Duration::days(7))
        .http_only(true)
        .finish();

    // 用于判断用户是管理员还是普通用户以进行身份验证
    let cookie_user_id = Cookie::build("USER_ID", user_info.id.to_string())
        .path("/")
        .max_age(Duration::days(7))
        .http_only(true)
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(cookie_access_token)
        .cookie(cookie_user_id)
        .body(format!("登陆成功! {}", user_info.login)))
}
