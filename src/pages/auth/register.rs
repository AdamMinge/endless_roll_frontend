use crate::router::Route;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Register)]
pub fn register() -> Html {
    let navigator = use_navigator().unwrap();
    let username = use_state(String::new);
    let email = use_state(String::new);
    let password = use_state(String::new);
    let password_confirm = use_state(String::new);
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

    let on_email_change = {
        let email = email.clone();
        Callback::from(move |e: web_sys::Event| {
            if let Some(input) = e
                .target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
            {
                email.set(input.value());
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

    let on_password_confirm_change = {
        let password_confirm = password_confirm.clone();
        Callback::from(move |e: web_sys::Event| {
            if let Some(input) = e
                .target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
            {
                password_confirm.set(input.value());
            }
        })
    };

    let on_register = {
        let username = username.clone();
        let email = email.clone();
        let password = password.clone();
        let password_confirm = password_confirm.clone();
        let error = error.clone();
        let loading = loading.clone();
        let navigator = navigator.clone();

        Callback::from(move |_| {
            let username = (*username).clone();
            let email = (*email).clone();
            let password = (*password).clone();
            let password_confirm = (*password_confirm).clone();
            let error = error.clone();
            let loading = loading.clone();
            let navigator = navigator.clone();

            if password != password_confirm {
                error.set(Some("Passwords do not match".to_string()));
                return;
            }

            if password.len() < 8 {
                error.set(Some("Password must be at least 8 characters".to_string()));
                return;
            }

            loading.set(true);
            error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                match crate::api::auth::register(&username, &email, &password).await {
                    Ok(_) => {
                        loading.set(false);
                        navigator.push(&Route::Login);
                    }
                    Err(err) => {
                        error.set(Some(format!("Registration failed: {}", err)));
                        loading.set(false);
                    }
                }
            });
        })
    };

    let on_login_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Login);
        })
    };

    html! {
        <div class="register-container" style="min-height: 100vh; display: flex; align-items: center; justify-content: center; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);">
            <div style="width: 100%; max-width: 400px; border-radius: 8px; box-shadow: 0 10px 40px rgba(0,0,0,0.3); background-color: white; overflow: hidden;">
                <div style="padding: 24px; background-color: #667eea; color: white;">
                    <h1 style="margin: 0; font-size: 24px;">{"Create Account"}</h1>
                </div>
                <div style="padding: 24px;">
                    <h1 style="text-align: center; margin-bottom: 24px; color: #333;">{"Join Endless Roll"}</h1>

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
                            placeholder="Choose a username (3-64 chars)"
                            value={(*username).clone()}
                            onchange={on_username_change}
                            style="width: 100%; padding: 10px; border: 1px solid #ddd; border-radius: 4px; font-size: 14px;"
                        />
                    </div>

                    <div style="margin-bottom: 16px;">
                        <label style="display: block; margin-bottom: 4px; font-weight: 500; color: #333;">{"Email"}</label>
                        <input
                            type="email"
                            placeholder="Enter your email"
                            value={(*email).clone()}
                            onchange={on_email_change}
                            style="width: 100%; padding: 10px; border: 1px solid #ddd; border-radius: 4px; font-size: 14px;"
                        />
                    </div>

                    <div style="margin-bottom: 16px;">
                        <label style="display: block; margin-bottom: 4px; font-weight: 500; color: #333;">{"Password"}</label>
                        <input
                            type="password"
                            placeholder="Enter password (min 8 chars)"
                            value={(*password).clone()}
                            onchange={on_password_change}
                            style="width: 100%; padding: 10px; border: 1px solid #ddd; border-radius: 4px; font-size: 14px;"
                        />
                    </div>

                    <div style="margin-bottom: 24px;">
                        <label style="display: block; margin-bottom: 4px; font-weight: 500; color: #333;">{"Confirm Password"}</label>
                        <input
                            type="password"
                            placeholder="Confirm your password"
                            value={(*password_confirm).clone()}
                            onchange={on_password_confirm_change}
                            style="width: 100%; padding: 10px; border: 1px solid #ddd; border-radius: 4px; font-size: 14px;"
                        />
                    </div>

                    <button
                        onclick={on_register}
                        disabled={*loading}
                        style="width: 100%; padding: 12px; background-color: #667eea; color: white; border: none; border-radius: 4px; font-size: 16px; font-weight: 500; cursor: pointer; transition: background-color 0.3s;"
                    >
                        {if *loading { "Creating account..." } else { "Register" }}
                    </button>

                    <div style="text-align: center; margin-top: 16px;">
                        <span style="color: #666;">{"Already have an account? "}</span>
                        <button
                            onclick={on_login_click}
                            style="background: none; border: none; color: #667eea; text-decoration: underline; cursor: pointer; font-size: 14px;"
                        >
                            {"Login here"}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
