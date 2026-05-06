use crate::models::{TokenResponse, User};
use gloo_net::http::Request;
use serde_json::json;
use web_sys::window;

const API_URL: &str = "http://localhost:8000/api/v1";

pub async fn register(username: &str, email: &str, password: &str) -> Result<User, String> {
    let body = json!({
        "username": username,
        "email": email,
        "password": password,
    });

    let response = Request::post(&format!("{}/auth/register", API_URL))
        .json(&body)
        .map_err(|e| format!("Request failed: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Send failed: {}", e))?;

    if response.ok() {
        response
            .json::<User>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    } else {
        Err(format!(
            "Registration failed with status: {}",
            response.status()
        ))
    }
}

pub async fn login(username: &str, password: &str) -> Result<TokenResponse, String> {
    let params = [("username", username), ("password", password)];
    let query_string = serde_urlencoded::to_string(params)
        .map_err(|e| format!("Failed to encode params: {}", e))?;

    let response = Request::post(&format!("{}/auth/token", API_URL))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(query_string)
        .map_err(|e| format!("Request failed: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Send failed: {}", e))?;

    if response.ok() {
        response
            .json::<TokenResponse>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    } else {
        Err(format!("Login failed with status: {}", response.status()))
    }
}

#[allow(dead_code)]
pub async fn get_current_user() -> Result<User, String> {
    match window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.get_item("auth_token").ok().flatten())
    {
        Some(token) => {
            let response = Request::get(&format!("{}/auth/me", API_URL))
                .header("Authorization", &format!("Bearer {}", token))
                .send()
                .await
                .map_err(|e| format!("Send failed: {}", e))?;

            if response.ok() {
                response
                    .json::<User>()
                    .await
                    .map_err(|e| format!("Failed to parse response: {}", e))
            } else {
                Err(format!(
                    "Get user failed with status: {}",
                    response.status()
                ))
            }
        }
        None => Err("No token found".to_string()),
    }
}
