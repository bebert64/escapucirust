use crate::{rooms::*, GlobalState};

use yew::prelude::*;

#[function_component(Component)]
pub(crate) fn html() -> Html {
    use Rooms::*;
    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    let room = match state.house_state.current_room {
        DiningRoomFaceUp => html! {<dining_room_face_up::Component />},
        Drawers => html! {<drawers::Component />},
        ElectricalPanel => html! {<electrical_panel::Component />},
        FirstFloorFaceDown => html! {<first_floor_face_down::Component />},
        Frigogidaire => html! {<frigogidaire::Component />},
        HallFaceUp => html! {<hall_face_up::Component />},
        HallFaceDown => html! {<hall_face_down::Component />},
        KitchenFaceDown => html! {<kitchen_face_down::Component />},
        KitchenFaceLeft => html! {<kitchen_face_left::Component />},
        LivingRoomFaceRight => html! {<living_room_face_right::Component />},
        LivingRoomFaceUp => html! {<living_room_face_up::Component />},
        PoolFaceDown => html! {<pool_face_down::Component />},
        PoolFaceLeft => html! {<pool_face_left::Component />},
        RoomGui1 => html! {<room_gui1::Component />},
        RoomMart1 => html! {<room_mart1::Component />},
        RoomTiph1 => html! {<room_tiph1::Component />},
        RoomRom1 => html! {<room_rom1::Component />},
        StairsFaceUp => html! {<stairs_face_up::Component />},
    };
    html! {
        <div class="board_Room">
            {room}
            {
                if state.house_state.is_light_on {
                    html!{<div class="rooms_BlackVeil"></div>}
                } else {
                    html!{<></>}
                }
            }
        </ div>
    }
}
