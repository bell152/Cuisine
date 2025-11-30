use yew::{html, Html};
use yew_router::Routable;
use crate::pages::{home::Home, privacy_policy::Privacy};




#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/privacy")]
    Privacy,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html!{ <Home/> },
        Route::Privacy => html!{ <Privacy/> }
    }
}
