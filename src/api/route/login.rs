use poem::{Response, Result};
use poem_openapi::{payload::Json, payload::PlainText, OpenApi};

use crate::api::ApiTag;
use crate::model::dto::User;
use crate::util::EmptyError;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::BTreeMap;

pub struct LoginRoute;

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

fn get_secret() -> Vec<u8> {
    std::env::var("JWT_SECRET").unwrap().into_bytes()
}

pub fn get_jwt_for_user(user: &User) -> Result<String, jwt::Error> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(&get_secret())?;
    let claims = BTreeMap::from([("sub", user.username.clone())]);

    let token = claims.sign_with_key(&key)?;
    Ok(token)
}

#[OpenApi]
impl LoginRoute {
    #[oai(path = "/login", method = "post", tag = ApiTag::Login)]
    pub async fn login(
        &self,
        user: Json<User>,
    ) -> Result<PlainText<String>, poem::Error> {
        log::info!("Received login request...");

        // NOTE(nrydanov): Temporary solution, expected to use prod DB
        match user.username.as_str() {
            "admin" => {
                if user.password == "admin" {
                    let result = get_jwt_for_user(&user);
                    // tmp.map_err(poem::error::InternalServerError)
                    match result {
                        Ok(token) => {
                            log::info!("Logged as admin");
                            Result::Ok(PlainText(token))
                        }
                        Err(msg) => {
                            Result::Err(poem::error::InternalServerError(msg))
                        }
                    }
                } else {
                    poem::Result::Err(poem::error::Forbidden(EmptyError))
                }
            }
            "user" => {
                if user.password == "user" {
                    let result = get_jwt_for_user(&user);
                    // tmp.map_err(poem::error::InternalServerError)
                    match result {
                        Ok(token) => {
                            log::info!("Logged as user");
                            Result::Ok(PlainText(token))
                        }
                        Err(msg) => {
                            Result::Err(poem::error::InternalServerError(msg))
                        }
                    }
                } else {
                    poem::Result::Err(poem::error::Forbidden(EmptyError))
                }
            }
            _ => {
                poem::Result::Err(poem::error::InternalServerError(EmptyError))
            }
        }
    }
}

struct MyError;

impl poem::error::ResponseError for MyError {
    fn status(&self) -> poem::http::StatusCode {
        poem::http::StatusCode::FORBIDDEN
    }

    fn as_response(&self) -> Response {
        let body = poem::Body::from_json(serde_json::json!({
            "message": "aboba"
        }))
        .unwrap();
        Response::builder().status(self.status()).body(body)
    }
}
