use crate::router::Route;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(CampaignDetail)]
pub fn campaign_detail(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let campaign = use_state(|| Option::<crate::models::Campaign>::None);
    let characters = use_state(Vec::<crate::models::Character>::new);
    let npcs = use_state(Vec::<crate::models::Npc>::new);
    let sessions = use_state(Vec::<crate::models::GameSession>::new);
    let search_query = use_state(String::new);
    let search_type = use_state(|| "all".to_string());

    let id = props.id.clone();

    use_effect_with(id.clone(), {
        let campaign = campaign.clone();
        let characters = characters.clone();
        let npcs = npcs.clone();
        let sessions = sessions.clone();
        move |id| {
            let id_clone = id.clone();
            let campaign = campaign.clone();
            let characters = characters.clone();
            let npcs = npcs.clone();
            let sessions = sessions.clone();

            wasm_bindgen_futures::spawn_local(async move {
                // Load campaign
                if let Ok(camp) = crate::api::campaigns::get_campaign(&id_clone).await {
                    campaign.set(Some(camp));
                }

                // Load characters
                if let Ok(chars) = crate::api::characters::get_campaign_characters(&id_clone).await
                {
                    characters.set(chars);
                }

                // Load NPCs
                if let Ok(npcs_data) = crate::api::npcs::get_campaign_npcs(&id_clone).await {
                    npcs.set(npcs_data);
                }

                // Load sessions
                if let Ok(sessions_data) =
                    crate::api::sessions::get_campaign_sessions(&id_clone).await
                {
                    sessions.set(sessions_data);
                }
            });

            || ()
        }
    });

    let on_back = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Home);
        })
    };

    let on_start_session = {
        let navigator = navigator.clone();
        let id = id.clone();
        Callback::from(move |_| {
            let id = id.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(session) = crate::api::sessions::create_session(&id).await {
                    navigator.push(&Route::GameSession {
                        session_id: session.id.to_string(),
                    });
                }
            });
        })
    };

    let on_search_query_change = {
        let search_query = search_query.clone();
        Callback::from(move |event: web_sys::Event| {
            if let Some(input) = event
                .target()
                .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
            {
                search_query.set(input.value());
            }
        })
    };

    let on_search_type_change = {
        let search_type = search_type.clone();
        Callback::from(move |event: web_sys::Event| {
            if let Some(select) = event
                .target()
                .and_then(|target| target.dyn_into::<web_sys::HtmlSelectElement>().ok())
            {
                search_type.set(select.value());
            }
        })
    };

    html! {
        <div style="min-height: 100vh; background-color: #f5f5f5;">
            // Header
            <div style="background-color: #1a1a1a; color: white; padding: 16px 24px; display: flex; justify-content: space-between; align-items: center;">
                <button
                    onclick={on_back}
                    style="padding: 8px 16px; background-color: #667eea; color: white; border: none; border-radius: 4px; cursor: pointer;"
                >
                    {"← Back"}
                </button>
            </div>

            <div style="max-width: 1400px; margin: 0 auto; padding: 24px;">
                {if let Some(campaign) = (*campaign).as_ref() {
                    html! {
                        <>
                            <div style="background-color: white; padding: 24px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); margin-bottom: 24px;">
                                <h2 style="margin: 0 0 8px 0; color: #333;">{&campaign.name}</h2>
                                <p style="margin: 0 0 16px 0; color: #666;">{campaign.description.clone().unwrap_or_default()}</p>
                                <button
                                    onclick={on_start_session}
                                    style="padding: 12px 24px; background-color: #27ae60; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 16px; font-weight: 500;"
                                >
                                    {"▶ Start Session"}
                                </button>
                            </div>

                            // Search & Browse Section
                            <div style="background-color: white; padding: 24px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); margin-bottom: 24px;">
                                <h3 style="margin: 0 0 16px 0; color: #333;">{"Search & Browse"}</h3>

                                <div style="display: flex; gap: 12px; margin-bottom: 16px;">
                                    <input
                                        type="text"
                                        placeholder="Search characters, NPCs, locations..."
                                        value={(*search_query).clone()}
                                        onchange={on_search_query_change}
                                        style="flex: 1; padding: 10px; border: 1px solid #ddd; border-radius: 4px;"
                                    />
                                    <select
                                        value={(*search_type).clone()}
                                        onchange={on_search_type_change}
                                        style="padding: 10px; border: 1px solid #ddd; border-radius: 4px;"
                                    >
                                        <option value="all">{"All"}</option>
                                        <option value="characters">{"Characters"}</option>
                                        <option value="npcs">{"NPCs"}</option>
                                        <option value="locations">{"Locations"}</option>
                                        <option value="sessions">{"Sessions"}</option>
                                    </select>
                                </div>

                                // Characters
                                <div style="margin-bottom: 24px;">
                                    <h4 style="color: #667eea; margin-bottom: 12px;">{"Characters"}{" ("}{characters.len()}{")"}</h4>
                                    <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(250px, 1fr)); gap: 12px;">
                                        {(*characters).iter().map(|char| {
                                            html! {
                                                <div
                                                    key={char.id.to_string()}
                                                    style="background-color: #f9f9f9; padding: 12px; border-radius: 4px; border-left: 3px solid #667eea;"
                                                >
                                                    <p style="margin: 0; font-weight: 500; color: #333;">{&char.name}</p>
                                                    <p style="margin: 4px 0 0 0; font-size: 12px; color: #666;">
                                                        {&char.species}{" - "}{&char.career.clone().unwrap_or_default()}
                                                    </p>
                                                </div>
                                            }
                                        }).collect::<Html>()}
                                    </div>
                                </div>

                                // NPCs
                                <div style="margin-bottom: 24px;">
                                    <h4 style="color: #e74c3c; margin-bottom: 12px;">{"NPCs"}{" ("}{npcs.len()}{")"}</h4>
                                    <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(250px, 1fr)); gap: 12px;">
                                        {(*npcs).iter().map(|npc| {
                                            html! {
                                                <div
                                                    key={npc.id.to_string()}
                                                    style="background-color: #f9f9f9; padding: 12px; border-radius: 4px; border-left: 3px solid #e74c3c;"
                                                >
                                                    <p style="margin: 0; font-weight: 500; color: #333;">{&npc.name}</p>
                                                    <p style="margin: 4px 0 0 0; font-size: 12px; color: #666;">
                                                        {&npc.role.clone().unwrap_or_default()}
                                                    </p>
                                                </div>
                                            }
                                        }).collect::<Html>()}
                                    </div>
                                </div>

                                // Sessions
                                <div>
                                    <h4 style="color: #f39c12; margin-bottom: 12px;">{"Sessions"}{" ("}{sessions.len()}{")"}</h4>
                                    <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(250px, 1fr)); gap: 12px;">
                                        {(*sessions).iter().map(|session| {
                                            html! {
                                                <div
                                                    key={session.id.to_string()}
                                                    style="background-color: #f9f9f9; padding: 12px; border-radius: 4px; border-left: 3px solid #f39c12;"
                                                >
                                                    <p style="margin: 0; font-weight: 500; color: #333;">{"Session "}{session.session_number}</p>
                                                    <p style="margin: 4px 0 0 0; font-size: 12px; color: #666;">
                                                        {"Status: "}{&session.status}
                                                    </p>
                                                </div>
                                            }
                                        }).collect::<Html>()}
                                    </div>
                                </div>
                            </div>
                        </>
                    }
                } else {
                    html! {
                        <div style="text-align: center; padding: 40px; color: #999;">
                            <p>{"Loading campaign..."}</p>
                        </div>
                    }
                }}
            </div>
        </div>
    }
}
