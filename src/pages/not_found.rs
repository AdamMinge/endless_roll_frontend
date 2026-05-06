use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div style="min-height: 100vh; display: flex; align-items: center; justify-content: center; background-color: #f5f5f5;">
            <div style="text-align: center;">
                <h1 style="font-size: 48px; color: #333; margin: 0;">{"404"}</h1>
                <p style="color: #666; margin-top: 16px; font-size: 18px;">{"Page not found"}</p>
            </div>
        </div>
    }
}
