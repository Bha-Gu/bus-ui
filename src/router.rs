use crate::components::pages::location::Location;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/bus")]
    Location,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Location => {
            html! {
                <Location />
            }
        }
    }
}
