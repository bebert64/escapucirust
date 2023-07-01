mod current_room;
mod narration;

use {current_room::CurrentRoom, narration::Narration};

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
            // <Map />
        </div>
        </>
    }
}
