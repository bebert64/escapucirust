mod gaming_board;
mod intro;
mod onclick_listener;
mod outro;
mod rooms;
mod start_screen;
mod store;

use {onclick_listener::add as add_onclick_listener, store::GlobalState};

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
                &Intro => html! {<intro::Component />},
                &Starting => html! {<start_screen::Component />},
                &Playing => html! {<gaming_board::Component />},
                &Outro => html! {<outro::Component />},
            }
        }
        </div>
    }
}

fn main() {
    yew::Renderer::<AppWithContext>::new().render();
}
