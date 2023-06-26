mod current_room;

use current_room::Component as CurrentRoom;

use crate::{GameStatus, GlobalState, StateAction};

use yew::prelude::*;

#[function_component(Component)]
pub(crate) fn html() -> Html {
    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    html! {
      <>
        <div class="board_Board">
            // <ItemOpened />
            <div class="board_Room">
            <CurrentRoom />
            </div>
            // <Inventory />
            // <Narration />
            // <Map />
        </div>
      </>
    }
}
