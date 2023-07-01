use crate::{rooms::*, GlobalState};

use yew::prelude::*;

#[function_component(CurrentRoom)]
pub(crate) fn html() -> Html {
    use Rooms::*;
    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    let room = match state.house.current_room {
        DiningRoomFaceUp => html! {<dining_room_face_up::Room />},
        Drawers => html! {<drawers::Room />},
        ElectricalPanel => html! {<electrical_panel::Room />},
        FirstFloorFaceDown => html! {<first_floor_face_down::Room />},
        Frigogidaire => html! {<frigogidaire::Room />},
        HallFaceUp => html! {<hall_face_up::Room />},
        HallFaceDown => html! {<hall_face_down::Room />},
        KitchenFaceDown => html! {<kitchen_face_down::Room />},
        KitchenFaceLeft => html! {<kitchen_face_left::Room />},
        LivingRoomFaceRight => html! {<living_room_face_right::Room />},
        LivingRoomFaceUp => html! {<living_room_face_up::Room />},
        PoolFaceDown => html! {<pool_face_down::Room />},
        PoolFaceLeft => html! {<pool_face_left::Room />},
        RoomGui1 => html! {<room_gui1::Room />},
        RoomMart1 => html! {<room_mart1::Room />},
        RoomTiph1 => html! {<room_tiph1::Room />},
        RoomRom1 => html! {<room_rom1::Room />},
        StairsFaceUp => html! {<stairs_face_up::Room />},
    };
    html! {
        <div class="board_Room">
            {room}
            {
                if state.house.is_light_on {
                    html!{<></>}
                } else {
                    html!{<div class="rooms_BlackVeil"></div>}
                }
            }
        </ div>
    }
}
