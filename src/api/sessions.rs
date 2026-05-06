use crate::models::GameSession;
use gloo_net::http::Request;
use web_sys::window;

const API_URL: &str = "http://localhost:8000/api/v1";

fn get_auth_header() -> Result<String, String> {
    window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.get_item("auth_token").ok().flatten())
        .ok_or_else(|| "No token found".to_string())
}

pub async fn get_campaign_sessions(campaign_id: &str) -> Result<Vec<GameSession>, String> {
    let token = get_auth_header()?;

    let response = Request::get(&format!("{}/campaigns/{}/sessions", API_URL, campaign_id))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Send failed: {}", e))?;

    if response.ok() {
        response
            .json::<Vec<GameSession>>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    } else {
        Err(format!(
            "Get sessions failed with status: {}",
            response.status()
        ))
    }
}

pub async fn create_session(campaign_id: &str) -> Result<GameSession, String> {
    let token = get_auth_header()?;

    let response = Request::post(&format!("{}/campaigns/{}/sessions", API_URL, campaign_id))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Send failed: {}", e))?;

    if response.ok() {
        response
            .json::<GameSession>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    } else {
        Err(format!(
            "Create session failed with status: {}",
            response.status()
        ))
    }
}
