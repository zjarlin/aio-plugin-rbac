use aio_plugin_rbac_model::{
    AccessControlErrorResponse, AccessControlResponse, AccessControlView, AssignRoleRequest,
    CreateRoleRequest, CreateUserRequest,
};

pub async fn load() -> Result<AccessControlView, String> {
    let response = gloo_net::http::Request::get("/api/rbac")
        .send()
        .await
        .map_err(|error| error.to_string())?;
    decode(response).await
}

pub async fn create_user(request: CreateUserRequest) -> Result<(), String> {
    post("/api/rbac/users", &request).await
}

pub async fn create_role(request: CreateRoleRequest) -> Result<(), String> {
    post("/api/rbac/roles", &request).await
}

pub async fn assign(request: AssignRoleRequest) -> Result<(), String> {
    post("/api/rbac/assignments", &request).await
}

async fn post<T: serde::Serialize>(path: &str, body: &T) -> Result<(), String> {
    let response = gloo_net::http::Request::post(path)
        .json(body)
        .map_err(|error| error.to_string())?
        .send()
        .await
        .map_err(|error| error.to_string())?;
    if response.ok() {
        Ok(())
    } else {
        decode_error(response).await
    }
}

async fn decode(response: gloo_net::http::Response) -> Result<AccessControlView, String> {
    if !response.ok() {
        return decode_error(response).await;
    }
    response
        .json::<AccessControlResponse<AccessControlView>>()
        .await
        .map(|response| response.data)
        .map_err(|error| error.to_string())
}

async fn decode_error<T>(response: gloo_net::http::Response) -> Result<T, String> {
    let body = response.text().await.unwrap_or_default();
    Err(serde_json::from_str::<AccessControlErrorResponse>(&body)
        .map(|response| response.error)
        .unwrap_or(body))
}

pub fn reload() {
    if let Some(window) = web_sys::window() {
        let _ = window.location().reload();
    }
}
