use ui::components;
use ui::game;
use ui::pages;

use yew::prelude::*;
use yew_router::prelude::*;
use pages::{local_game::LocalGame, online_game::OnlineGame};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/local")]
    LocalGame,
    #[at("/online/:game_id")]
    OnlineGame { game_id: String },
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{"Welcome to Gomoku"}</h1> },
        Route::LocalGame => html! { <LocalGame /> },
        Route::OnlineGame { game_id } => html! { <OnlineGame game_id={game_id} /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}