use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub is_active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Campaign {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub setting_id: String,
    pub is_active: bool,
    pub world_state: serde_json::Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Character {
    pub id: Uuid,
    pub campaign_id: Uuid,
    pub name: String,
    pub species: String,
    pub career: Option<String>,
    pub career_tier: i32,
    pub background: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Npc {
    pub id: Uuid,
    pub campaign_id: Uuid,
    pub name: String,
    pub role: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameSession {
    pub id: Uuid,
    pub campaign_id: Uuid,
    pub status: String,
    pub session_number: i32,
    pub summary: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: Uuid,
    pub author: String,
    pub content: String,
    pub timestamp: DateTime<Local>,
    pub message_type: String,
}
