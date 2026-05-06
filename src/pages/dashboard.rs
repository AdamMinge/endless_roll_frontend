use crate::router::Route;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let navigator = use_navigator().unwrap();
    let campaigns = use_state(Vec::<crate::models::Campaign>::new);

    use_effect_with((), {
        let campaigns = campaigns.clone();
        move |_| {
            let campaigns = campaigns.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match crate::api::campaigns::get_user_campaigns().await {
                    Ok(data) => campaigns.set(data),
                    Err(err) => {
                        gloo_console::error!(format!("Failed to load campaigns: {}", err));
                    }
                }
            });
            || ()
        }
    });

    let on_create_campaign = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            // TODO: Open create campaign modal
            navigator.push(&Route::Campaigns);
        })
    };

    let on_campaign_click = {
        let navigator = navigator.clone();
        Callback::from(move |campaign_id: String| {
            navigator.push(&Route::CampaignDetail { id: campaign_id });
        })
    };

    let on_logout = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            if let Some(loc_storage) = window().and_then(|w| w.local_storage().ok()).flatten() {
                let _ = loc_storage.remove_item("auth_token");
            }
            navigator.push(&Route::Login);
        })
    };

    html! {
        <div style="min-height: 100vh; background-color: #f5f5f5;">
            // Header
            <div style="background-color: #1a1a1a; color: white; padding: 16px 24px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
                <div style="display: flex; justify-content: space-between; align-items: center; max-width: 1400px; margin: 0 auto;">
                    <h1 style="margin: 0; font-size: 24px;">{"⚔️ Endless Roll"}</h1>
                    <button
                        onclick={on_logout}
                        style="padding: 8px 16px; background-color: #e74c3c; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px;"
                    >
                        {"Logout"}
                    </button>
                </div>
            </div>

            // Main Content
            <div style="max-width: 1400px; margin: 0 auto; padding: 24px;">
                <div style="margin-bottom: 32px;">
                    <h2 style="margin-bottom: 16px; color: #333;">{"Welcome to Endless Roll"}</h2>
                    <p style="color: #666; margin-bottom: 24px;">{"Manage your campaigns and begin epic adventures"}</p>

                    <button
                        onclick={on_create_campaign}
                        style="padding: 12px 24px; background-color: #667eea; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 16px; font-weight: 500;"
                    >
                        {"+ Create New Campaign"}
                    </button>
                </div>

                <h3 style="margin-bottom: 16px; color: #333;">{"Your Campaigns"}</h3>
                <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 16px;">
                    {(*campaigns).iter().map(|campaign| {
                        let campaign_id = campaign.id.to_string();
                        let on_click = {
                            let on_campaign_click = on_campaign_click.clone();
                            let campaign_id = campaign_id.clone();
                            Callback::from(move |_| {
                                on_campaign_click.emit(campaign_id.clone());
                            })
                        };

                        html! {
                            <div
                                key={campaign_id.clone()}
                                onclick={on_click}
                                style="background-color: white; padding: 16px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); cursor: pointer; transition: transform 0.3s, box-shadow 0.3s; border-left: 4px solid #667eea;"
                            >
                                <h4 style="margin: 0 0 8px 0; color: #333;">{&campaign.name}</h4>
                                <p style="margin: 0 0 12px 0; color: #666; font-size: 14px; height: 40px; overflow: hidden;">
                                    {campaign.description.clone().unwrap_or_else(|| "No description".to_string())}
                                </p>
                                <div style="font-size: 12px; color: #999;">
                                    <p style="margin: 4px 0;">{"Setting: "}{&campaign.setting_id}</p>
                                    <p style="margin: 4px 0;">{"Status: "}{if campaign.is_active { "Active" } else { "Inactive" }}</p>
                                </div>
                            </div>
                        }
                    }).collect::<Html>()}
                </div>

                {if (*campaigns).is_empty() {
                    html! {
                        <div style="text-align: center; padding: 40px; color: #999;">
                            <p>{"No campaigns yet. Create one to get started!"}</p>
                        </div>
                    }
                } else {
                    html! {}
                }}
            </div>
        </div>
    }
}
