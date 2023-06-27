mod current_room;
mod narration;

use {current_room::Component as CurrentRoom, narration::Component as Narration};

use yew::prelude::*;

#[function_component(Component)]
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
