use crate::{GameStatus, GlobalState, StateAction};

use yew::prelude::*;

#[function_component(Component)]
pub(crate) fn html() -> Html {
    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    let onclick =
        { Callback::from(move |_| state.dispatch(StateAction::Status(GameStatus::Starting))) };
    html! {
      <>
        <h1>{"Gagné !!! Bravo Doudou !!!"}</h1>
        <button {onclick}>{"Recommencer"}</button>
      </>
    }
}
