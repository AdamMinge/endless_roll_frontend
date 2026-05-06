use crate::models::Campaign;
use gloo_net::http::Request;
use serde_json::json;
use web_sys::window;

const API_URL: &str = "http://localhost:8000/api/v1";

fn get_auth_header() -> Result<String, String> {
    window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.get_item("auth_token").ok().flatten())
        .ok_or_else(|| "No token found".to_string())
}

pub async fn get_user_campaigns() -> Result<Vec<Campaign>, String> {
    let token = get_auth_header()?;

    let response = Request::get(&format!("{}/campaigns", API_URL))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Send failed: {}", e))?;

    if response.ok() {
        response
            .json::<Vec<Campaign>>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    } else {
        Err(format!(
            "Get campaigns failed with status: {}",
            response.status()
        ))
    }
}

pub async fn get_campaign(campaign_id: &str) -> Result<Campaign, String> {
    let token = get_auth_header()?;

    let response = Request::get(&format!("{}/campaigns/{}", API_URL, campaign_id))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Send failed: {}", e))?;

    if response.ok() {
        response
            .json::<Campaign>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    } else {
        Err(format!(
            "Get campaign failed with status: {}",
            response.status()
        ))
    }
}

#[allow(dead_code)]
pub async fn create_campaign(name: &str, description: &str) -> Result<Campaign, String> {
    let token = get_auth_header()?;
    let body = json!({
        "name": name,
        "description": description,
        "setting_id": "old_world",
    });

    let response = Request::post(&format!("{}/campaigns", API_URL))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&body)
        .map_err(|e| format!("Request failed: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Send failed: {}", e))?;

    if response.ok() {
        response
            .json::<Campaign>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    } else {
        Err(format!(
            "Create campaign failed with status: {}",
            response.status()
        ))
    }
}
