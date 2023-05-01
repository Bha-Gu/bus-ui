use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod router;
mod stores;

use crate::router::{switch, Route};
use components::unit::main_title::MainTitle;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <MainTitle label={"Bus"}/>
                <BrowserRouter>
                    <Link<Route> to={Route::Location}>{"Location"}</Link<Route>>
                    {"   "}
                    <p />
                    <Switch<Route> render={switch} />
                </BrowserRouter>

        </>
    }
}
