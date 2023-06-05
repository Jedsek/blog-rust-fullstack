use crate::error::CustomError;
use actix_web::FromRequest;
use serde::Deserialize;
use std::{future::Future, pin::Pin};

const ADMIN_GITHUB_ID: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/admin_github_id.txt"));

#[derive(Debug, Clone, Deserialize)]
pub struct Login {
    pub code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccessToekn {
    pub access_token: String,
}

#[derive(Debug, Clone)]
pub struct User {
    pub user_id: String,
    pub access_token: String,
}

#[derive(Debug, Clone)]
pub struct Admin {
    pub access_token: String,
}

impl FromRequest for User {
    type Error = CustomError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let cookie_access_token = req.cookie("ACCESS_TOKEN");
            let cookie_user_id = req.cookie("USER_ID");

            match (cookie_user_id.as_ref(), cookie_access_token.as_ref()) {
                (Some(user_id), Some(access_token)) => {
                    let user_id = user_id.value().to_string();
                    let access_token = access_token.value().to_string();
                    Ok(User {
                        user_id,
                        access_token,
                    })
                }
                _ => Err(CustomError::AuthFailed("你还没有登陆过, 请登陆".into())),
            }
        })
    }
}

impl FromRequest for Admin {
    type Error = CustomError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let cookie_access_token = req.cookie("ACCESS_TOKEN");
            let cookie_user_id = req.cookie("USER_ID");

            match (cookie_user_id.as_ref(), cookie_access_token.as_ref()) {
                (Some(user_id), Some(access_token)) => {
                    let user_id = user_id.value().to_string();
                    let access_token = access_token.value().to_string();

                    if user_id == ADMIN_GITHUB_ID {
                        Ok(Admin { access_token })
                    } else {
                        Err(CustomError::AuthFailed("你不是管理员, 权限不足".into()))
                    }
                }
                _ => Err(CustomError::AuthFailed("你还没有登陆过, 请登陆".into())),
            }
        })
    }
}
