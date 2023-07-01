#![allow(unused)]

mod gaming_board;
mod intro;
mod items;
mod outro;
mod rooms;
mod start_screen;
mod store;

use store::GlobalState;

use yew::prelude::*;

#[function_component(AppWithContext)]
fn app_with_context() -> Html {
    let state = use_reducer(|| GlobalState::default());
    html! {
        <ContextProvider<UseReducerHandle<GlobalState>> context={state.clone()}>
            <App />
        </ ContextProvider<UseReducerHandle<GlobalState>>>
    }
}

#[function_component(App)]
fn app() -> Html {
    use store::game_status::GameStatus::*;

    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    html! {
        <div>
        {
            match &state.game_status {
                &Intro => html! {<intro::Intro />},
                &Starting => html! {<start_screen::StartScreen />},
                &Playing => html! {<gaming_board::GamingBoard />},
                &Outro => html! {<outro::Outro />},
            }
        }
        </div>
    }
}

fn main() {
    yew::Renderer::<AppWithContext>::new().render();
}
