use aio_plugin_rbac_model::{AccessControlErrorResponse, AccessControlResponse, AccessControlView};
use gloo_net::http::Request;
use serde::Serialize;

pub async fn load() -> Result<AccessControlView, String> {
    let response = Request::get("/api/rbac")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !response.ok() {
        return Err(error(response).await);
    }
    response
        .json::<AccessControlResponse<AccessControlView>>()
        .await
        .map(|r| r.data)
        .map_err(|e| e.to_string())
}

pub async fn save<T: Serialize>(path: &str, body: &T, create: bool) -> Result<(), String> {
    let request = if create {
        Request::post(path)
    } else {
        Request::put(path)
    };
    let response = request
        .json(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if response.ok() {
        Ok(())
    } else {
        Err(error(response).await)
    }
}

pub async fn delete(path: &str) -> Result<(), String> {
    let response = Request::delete(path)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if response.ok() {
        Ok(())
    } else {
        Err(error(response).await)
    }
}

async fn error(response: gloo_net::http::Response) -> String {
    let body = response.text().await.unwrap_or_default();
    serde_json::from_str::<AccessControlErrorResponse>(&body)
        .map(|r| r.error)
        .unwrap_or(body)
}
