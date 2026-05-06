mod api;
mod components;
mod models;
mod pages;
mod router;
mod store;

use yew::prelude::*;
use yew_router::prelude::*;

use router::Route;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <pages::dashboard::Dashboard /> },
        Route::Login => html! { <pages::auth::Login /> },
        Route::Register => html! { <pages::auth::Register /> },
        Route::Campaigns => html! { <pages::campaigns::Campaigns /> },
        Route::CampaignDetail { id } => html! { <pages::campaign_detail::CampaignDetail {id} /> },
        Route::GameSession { session_id } => {
            html! { <pages::game_session::GameSession {session_id} /> }
        }
        Route::NotFound => html! { <pages::not_found::NotFound /> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
