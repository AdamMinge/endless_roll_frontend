use crate::models::Character;
use gloo_net::http::Request;
use web_sys::window;

const API_URL: &str = "http://localhost:8000/api/v1";

fn get_auth_header() -> Result<String, String> {
    window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.get_item("auth_token").ok().flatten())
        .ok_or_else(|| "No token found".to_string())
}

pub async fn get_campaign_characters(campaign_id: &str) -> Result<Vec<Character>, String> {
    let token = get_auth_header()?;

    let response = Request::get(&format!("{}/campaigns/{}/characters", API_URL, campaign_id))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Send failed: {}", e))?;

    if response.ok() {
        response
            .json::<Vec<Character>>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    } else {
        Err(format!(
            "Get characters failed with status: {}",
            response.status()
        ))
    }
}
