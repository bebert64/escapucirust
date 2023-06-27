mod current_room;

use current_room::Component as CurrentRoom;

use yew::prelude::*;

#[function_component(Component)]
pub(crate) fn html() -> Html {
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
