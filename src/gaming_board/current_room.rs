use crate::{rooms::*, GlobalState};

use yew::prelude::*;

#[function_component(Component)]
pub(crate) fn html() -> Html {
    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    let room = match state.house_state.current_room {
        Rooms::HallFaceUp => html! {<hall_face_up::Component />},
        Rooms::HallFaceDown => html! {<hall_face_down::Component />},
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
