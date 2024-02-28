use std::fmt::Debug;

use base64::prelude::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub const BACKEND_SITE: &str = "http://localhost:8080";

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Person {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum DbPrivilege {
    CanRead,
    CanWrite,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub privileges: Vec<DbPrivilege>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LoggingUser {
    pub username: String,
    pub password: String,
}

// https://github.com/jetli/rust-yew-realworld-example-app/blob/31160005628cc6fb5ad6c9ecc39fec04a0edb0fb/crates/conduit-wasm/src/services/requests.rs#L42
pub async fn request<B, T>(
    method: reqwest::Method,
    url: String,
    auth_user: Option<LoggingUser>,
    body: B,
) -> Result<T, String>
where
    T: DeserializeOwned + 'static + Debug,
    B: Serialize + Debug,
{
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
    let url = format!("{}{}", BACKEND_SITE, url);
    let mut builder = reqwest::Client::new()
        .request(method, url)
        .header("Content-Type", "application/json");

    if let Some(u) = auth_user {
        let auth_string = "Basic ".to_string()
            + &BASE64_STANDARD.encode(format!("{}:{}", u.username, u.password));
        builder = builder.header("authorization", auth_string);
    }
    if allow_body {
        builder = builder.json(&body)
    }

    let resp = builder.send().await;

    if let Ok(res) = resp {
        if res.status().is_success() {
            if let Ok(data) = res.json::<T>().await {
                Ok(data)
            } else {
                Err("deserialization failed.".to_string())
            }
        } else {
            Err(res.status().to_string())
        }
    } else {
        Err("request failed".to_string())
    }
}
