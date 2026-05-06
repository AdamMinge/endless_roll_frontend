use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/campaigns")]
    Campaigns,
    #[at("/campaigns/:id")]
    CampaignDetail { id: String },
    #[at("/session/:session_id")]
    GameSession { session_id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}
