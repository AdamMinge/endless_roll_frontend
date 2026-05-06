use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Campaigns)]
pub fn campaigns() -> Html {
    let navigator = use_navigator().unwrap();

    let on_back = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Home);
        })
    };

    html! {
        <div style="min-height: 100vh; background-color: #f5f5f5;">
            <div style="background-color: #1a1a1a; color: white; padding: 16px 24px;">
                <button
                    onclick={on_back}
                    style="padding: 8px 16px; background-color: #667eea; color: white; border: none; border-radius: 4px; cursor: pointer;"
                >
                    {"← Back"}
                </button>
            </div>

            <div style="max-width: 1400px; margin: 0 auto; padding: 24px;">
                <h2>{"Manage Campaigns"}</h2>
                <p>{"Campaign management coming soon..."}</p>
            </div>
        </div>
    }
}
