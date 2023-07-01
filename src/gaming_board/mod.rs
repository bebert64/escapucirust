mod current_room;
mod inventory;
mod map;
mod narration;
mod opened_item;

use {
    current_room::CurrentRoom, inventory::Inventory, map::Map, narration::Narration,
    opened_item::OpenedItem,
};

use yew::prelude::*;

#[function_component(GamingBoard)]
pub(crate) fn html() -> Html {
    html! {
        <>
        <div class="board_Board">
            <OpenedItem />
            <CurrentRoom />
            <Inventory />
            <Narration />
            <Map />
        </div>
        </>
    }
}
