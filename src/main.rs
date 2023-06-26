mod global_state;
mod intro;
mod start_screen;

use global_state::*;

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
    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    let game_status = &state.game_status;
    html! {
        <div class="App">
        {
            match game_status {
                &GameStatus::Intro => html! {<intro::Component />},
                &GameStatus::Starting => html! {<start_screen::Component />},
                _ => html! {<h1>{format!("{game_status:?}")}</h1>},
            }
        }
        </div>
    }
}

fn main() {
    yew::Renderer::<AppWithContext>::new().render();
}
