use crate::router::Route;
use wasm_bindgen::JsCast;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    let navigator = use_navigator().unwrap();
    let username = use_state(String::new);
    let password = use_state(String::new);
    let error = use_state(|| Option::<String>::None);
    let loading = use_state(bool::default);

    let on_username_change = {
        let username = username.clone();
        Callback::from(move |e: web_sys::Event| {
            if let Some(input) = e
                .target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
            {
                username.set(input.value());
            }
        })
    };

    let on_password_change = {
        let password = password.clone();
        Callback::from(move |e: web_sys::Event| {
            if let Some(input) = e
                .target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
            {
                password.set(input.value());
            }
        })
    };

    let on_login = {
        let username = username.clone();
        let password = password.clone();
        let error = error.clone();
        let loading = loading.clone();
        let navigator = navigator.clone();

        Callback::from(move |_| {
            let username = (*username).clone();
            let password = (*password).clone();
            let error = error.clone();
            let loading = loading.clone();
            let navigator = navigator.clone();

            loading.set(true);
            error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                // TODO: Call backend API for login
                // For now, mock the login
                match crate::api::auth::login(&username, &password).await {
                    Ok(token_response) => {
                        if let Some(loc_storage) =
                            window().and_then(|w| w.local_storage().ok()).flatten()
                        {
                            let _ =
                                loc_storage.set_item("auth_token", &token_response.access_token);
                        }
                        loading.set(false);
                        navigator.push(&Route::Home);
                    }
                    Err(err) => {
                        error.set(Some(format!("Login failed: {}", err)));
                        loading.set(false);
                    }
                }
            });
        })
    };

    let on_register_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Register);
        })
    };

    html! {
        <div class="login-container" style="min-height: 100vh; display: flex; align-items: center; justify-content: center; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);">
            <div style="width: 100%; max-width: 400px; border-radius: 8px; box-shadow: 0 10px 40px rgba(0,0,0,0.3); background-color: white; overflow: hidden;">
                <div style="padding: 24px; background-color: #667eea; color: white;">
                    <h1 style="margin: 0; font-size: 24px;">{"Endless Roll"}</h1>
                </div>
                <div style="padding: 24px;">
                    <h1 style="text-align: center; margin-bottom: 24px; color: #333;">{"Login to Endless Roll"}</h1>

                    {if let Some(err) = (*error).as_ref() {
                        html! {
                            <div style="background-color: #f8d7da; color: #721c24; padding: 12px; border-radius: 4px; margin-bottom: 16px; border: 1px solid #f5c6cb;">
                                {err}
                            </div>
                        }
                    } else {
                        html! {}
                    }}

                    <div style="margin-bottom: 16px;">
                        <label style="display: block; margin-bottom: 4px; font-weight: 500; color: #333;">{"Username"}</label>
                        <input
                            type="text"
                            placeholder="Enter your username"
                            value={(*username).clone()}
                            onchange={on_username_change}
                            style="width: 100%; padding: 10px; border: 1px solid #ddd; border-radius: 4px; font-size: 14px;"
                        />
                    </div>

                    <div style="margin-bottom: 24px;">
                        <label style="display: block; margin-bottom: 4px; font-weight: 500; color: #333;">{"Password"}</label>
                        <input
                            type="password"
                            placeholder="Enter your password"
                            value={(*password).clone()}
                            onchange={on_password_change}
                            style="width: 100%; padding: 10px; border: 1px solid #ddd; border-radius: 4px; font-size: 14px;"
                        />
                    </div>

                    <button
                        onclick={on_login}
                        disabled={*loading}
                        style="width: 100%; padding: 12px; background-color: #667eea; color: white; border: none; border-radius: 4px; font-size: 16px; font-weight: 500; cursor: pointer; transition: background-color 0.3s;"
                    >
                        {if *loading { "Logging in..." } else { "Login" }}
                    </button>

                    <div style="text-align: center; margin-top: 16px;">
                        <span style="color: #666;">{"Don't have an account? "}</span>
                        <button
                            onclick={on_register_click}
                            style="background: none; border: none; color: #667eea; text-decoration: underline; cursor: pointer; font-size: 14px;"
                        >
                            {"Register here"}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
