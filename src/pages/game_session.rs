use crate::models::ChatMessage;
use crate::router::Route;
use serde_json::json;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::WebSocket;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub session_id: String,
}

#[function_component(GameSession)]
pub fn game_session(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let messages = use_state(Vec::<ChatMessage>::new);
    let input_text = use_state(String::new);
    let websocket = use_state(|| Option::<WebSocket>::None);
    let connected = use_state(bool::default);
    let session_id = props.session_id.clone();
    let last_roll_result = use_state(|| Option::<String>::None);

    // Connect to websocket
    use_effect_with(session_id.clone(), {
        let websocket = websocket.clone();
        let messages = messages.clone();
        let connected = connected.clone();
        move |session_id| {
            let session_id = session_id.clone();
            let websocket = websocket.clone();
            let messages = messages.clone();
            let connected = connected.clone();

            wasm_bindgen_futures::spawn_local(async move {
                // TODO: Get actual API URL from config
                let ws_url = format!("ws://localhost:8000/api/v1/ws/game/{}", session_id);

                match WebSocket::new(&ws_url) {
                    Ok(ws) => {
                        websocket.set(Some(ws.clone()));

                        // On message handler
                        let on_message = {
                            let messages = messages.clone();
                            Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
                                if let Some(text) = event.data().as_string() {
                                    if let Ok(json) =
                                        serde_json::from_str::<serde_json::Value>(&text)
                                    {
                                        let msg_type =
                                            json.get("type").and_then(|v| v.as_str()).unwrap_or("");
                                        let content = json
                                            .get("content")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("");

                                        let mut msgs = (*messages).clone();
                                        msgs.push(ChatMessage {
                                            id: uuid::Uuid::new_v4(),
                                            author: if msg_type == "player_action" {
                                                "You".to_string()
                                            } else {
                                                "GM".to_string()
                                            },
                                            content: content.to_string(),
                                            timestamp: chrono::Local::now(),
                                            message_type: msg_type.to_string(),
                                        });
                                        messages.set(msgs);
                                    }
                                }
                            })
                                as Box<dyn Fn(web_sys::MessageEvent)>)
                        };

                        ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
                        on_message.forget();

                        // On open handler
                        let on_open = {
                            let connected = connected.clone();
                            Closure::wrap(Box::new(move |_: web_sys::Event| {
                                connected.set(true);
                                gloo_console::log!("WebSocket connected");
                            })
                                as Box<dyn Fn(web_sys::Event)>)
                        };

                        ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));
                        on_open.forget();

                        // On error handler
                        let on_error = Closure::wrap(Box::new(move |_: web_sys::Event| {
                            gloo_console::error!("WebSocket error");
                        })
                            as Box<dyn Fn(web_sys::Event)>);

                        ws.set_onerror(Some(on_error.as_ref().unchecked_ref()));
                        on_error.forget();

                        // On close handler
                        let on_close = {
                            let connected = connected.clone();
                            Closure::wrap(Box::new(move |_: web_sys::CloseEvent| {
                                connected.set(false);
                                gloo_console::log!("WebSocket closed");
                            })
                                as Box<dyn Fn(web_sys::CloseEvent)>)
                        };

                        ws.set_onclose(Some(on_close.as_ref().unchecked_ref()));
                        on_close.forget();
                    }
                    Err(_) => {
                        gloo_console::error!("Failed to create WebSocket");
                    }
                }
            });

            || ()
        }
    });

    let on_input_change = {
        let input_text = input_text.clone();
        Callback::from(move |event: web_sys::Event| {
            if let Some(input) = event
                .target()
                .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
            {
                input_text.set(input.value());
            }
        })
    };

    let on_send_message = {
        let input_text = input_text.clone();
        let websocket = websocket.clone();
        let last_roll_result = last_roll_result.clone();
        Callback::from(move |_| {
            let text = (*input_text).clone();
            if text.trim().is_empty() {
                return;
            }

            if let Some(ws) = (*websocket).as_ref() {
                let message = json!({
                    "type": "player_action",
                    "content": text,
                });
                if let Ok(msg_str) = serde_json::to_string(&message) {
                    ws.send_with_str(&msg_str).ok();
                    input_text.set(String::new());

                    // Check if message contains dice roll syntax (e.g., "roll 2d6")
                    if text.to_lowercase().contains("roll") {
                        // Simulate dice roll for display
                        let roll_result = "🎲 Rolled: 12 (2d6)".to_string();
                        last_roll_result.set(Some(roll_result));
                    }
                }
            }
        })
    };

    let on_roll_d20 = {
        let websocket = websocket.clone();
        Callback::from(move |_| {
            if let Some(ws) = (*websocket).as_ref() {
                let roll_value = (js_sys::Math::random() * 20.0 + 1.0).floor() as i32;
                let message = json!({
                    "type": "roll_request",
                    "dice_type": "d20",
                    "result": roll_value,
                });
                if let Ok(msg_str) = serde_json::to_string(&message) {
                    ws.send_with_str(&msg_str).ok();
                }
            }
        })
    };

    let on_back = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Home);
        })
    };

    html! {
        <div style="min-height: 100vh; background-color: #1a1a1a; display: flex; flex-direction: column;">
            // Header
            <div style="background-color: #0d0d0d; color: white; padding: 16px 24px; border-bottom: 2px solid #667eea; display: flex; justify-content: space-between; align-items: center;">
                <div>
                    <h2 style="margin: 0; color: white;">{"⚔️ Game Session"}</h2>
                    <p style={format!("margin: 4px 0 0 0; font-size: 14px; color: {};", if *connected { "#27ae60" } else { "#e74c3c" })}>
                        {if *connected { "🟢 Connected" } else { "🔴 Disconnected" }}
                    </p>
                </div>
                <button
                    onclick={on_back}
                    style="padding: 8px 16px; background-color: #667eea; color: white; border: none; border-radius: 4px; cursor: pointer;"
                >
                    {"← Back"}
                </button>
            </div>

            // Main content
            <div style="display: flex; flex: 1; overflow: hidden;">
                // Chat area
                <div style="flex: 1; display: flex; flex-direction: column; border-right: 1px solid #333;">
                    // Messages
                    <div style="flex: 1; overflow-y: auto; padding: 16px; background-color: #1a1a1a;">
                        {(*messages).iter().map(|msg| {
                            let is_player = msg.author == "You";
                            html! {
                                <div
                                    key={msg.id.to_string()}
                                    style={format!(
                                        "margin-bottom: 16px; display: flex; justify-content: {}; align-items: flex-start;",
                                        if is_player { "flex-end" } else { "flex-start" }
                                    )}
                                >
                                    <div
                                        style={format!(
                                            "max-width: 70%; padding: 12px 16px; border-radius: 8px; background-color: {}; color: white;",
                                            if is_player { "#667eea" } else { "#2a2a2a" }
                                        )}
                                    >
                                        <p style="margin: 0 0 4px 0; font-weight: 500; font-size: 12px; opacity: 0.7;">
                                            {&msg.author}
                                        </p>
                                        <p style="margin: 0; word-wrap: break-word;">{&msg.content}</p>
                                        <p style="margin: 4px 0 0 0; font-size: 11px; opacity: 0.6;">
                                            {msg.timestamp.format("%H:%M").to_string()}
                                        </p>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()}
                    </div>

                    // Roll result display
                    {if let Some(roll) = (*last_roll_result).as_ref() {
                        html! {
                            <div style="background-color: #2a2a2a; padding: 12px 16px; border-top: 1px solid #333; text-align: center; color: #f39c12;">
                                <div style="font-size: 20px; font-weight: bold; animation: bounce 0.5s;">
                                    {roll}
                                </div>
                            </div>
                        }
                    } else {
                        html! {}
                    }}

                    // Input area
                    <div style="padding: 16px; background-color: #0d0d0d; border-top: 1px solid #333;">
                        <div style="display: flex; gap: 8px; margin-bottom: 8px;">
                            <input
                                type="text"
                                placeholder="What do you do?"
                                value={(*input_text).clone()}
                                onchange={on_input_change}
                                style="flex: 1; padding: 12px; background-color: #2a2a2a; color: white; border: 1px solid #667eea; border-radius: 4px; font-size: 14px;"
                            />
                            <button
                                onclick={on_send_message}
                                disabled={!*connected}
                                style="padding: 12px 24px; background-color: #667eea; color: white; border: none; border-radius: 4px; cursor: pointer; font-weight: 500;"
                            >
                                {"Send"}
                            </button>
                        </div>
                        <div style="display: flex; gap: 8px;">
                            <button
                                onclick={on_roll_d20}
                                disabled={!*connected}
                                style="flex: 1; padding: 10px; background-color: #e74c3c; color: white; border: none; border-radius: 4px; cursor: pointer; font-weight: 500;"
                            >
                                {"🎲 Roll d20"}
                            </button>
                            <button
                                style="flex: 1; padding: 10px; background-color: #f39c12; color: white; border: none; border-radius: 4px; cursor: pointer; font-weight: 500;"
                            >
                                {"🎲 Roll d12"}
                            </button>
                            <button
                                style="flex: 1; padding: 10px; background-color: #27ae60; color: white; border: none; border-radius: 4px; cursor: pointer; font-weight: 500;"
                            >
                                {"🎲 Roll 2d6"}
                            </button>
                        </div>
                    </div>
                </div>

                // Side panel - Info
                <div style="width: 300px; background-color: #0d0d0d; border-left: 1px solid #333; padding: 16px; overflow-y: auto;">
                    <h3 style="color: #667eea; margin-top: 0;">{"Game Info"}</h3>
                    <div style="font-size: 12px; color: #aaa; line-height: 1.8;">
                        <p><strong>{"Session ID:"}</strong><br/>{&session_id}</p>
                        <p><strong>{"Connection:"}</strong><br/>{if *connected { "Active" } else { "Inactive" }}</p>
                        <p><strong>{"Messages:"}</strong><br/>{(*messages).len()}</p>
                    </div>

                    <h3 style="color: #667eea; margin-top: 24px;">{"Quick Actions"}</h3>
                    <div style="display: flex; flex-direction: column; gap: 8px;">
                        <button style="padding: 8px; background-color: #667eea; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 12px;">
                            {"📋 Character Sheet"}
                        </button>
                        <button style="padding: 8px; background-color: #667eea; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 12px;">
                            {"📜 Inventory"}
                        </button>
                        <button style="padding: 8px; background-color: #667eea; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 12px;">
                            {"🗺️ Map"}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
