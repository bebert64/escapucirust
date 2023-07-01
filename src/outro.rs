use crate::{store::game_status::display_start_menu, GlobalState};

use yew::prelude::*;

#[function_component(Outro)]
pub(crate) fn html() -> Html {
    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    let onclick = { Callback::from(move |_| state.dispatch(display_start_menu())) };
    html! {
      <>
        <h1>{"Gagné !!! Bravo Doudou !!!"}</h1>
        <button {onclick}>{"Recommencer"}</button>
      </>
    }
}
