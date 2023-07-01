mod current_room;
mod inventory;
mod item_opened;
mod map;
mod narration;

use {current_room::CurrentRoom, map::Map, narration::Narration};

use yew::prelude::*;

#[function_component(GamingBoard)]
pub(crate) fn html() -> Html {
    html! {
        <>
        <div class="board_Board">
            // <ItemOpened />
            <CurrentRoom />
            // <Inventory />
            <Narration />
            <Map />
        </div>
        </>
    }
}
