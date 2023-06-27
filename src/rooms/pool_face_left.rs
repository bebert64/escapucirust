use super::Rooms;

use crate::{
    add_onclick_listener,
    store::{house_state::set_current_room, narration::set_current_text},
    GlobalState,
};

use yew::prelude::*;

#[function_component(Component)]
pub(crate) fn html() -> Html {
    let my_room_ref = use_node_ref();
    let svg = yew::Html::from_html_unchecked(yew::AttrValue::from(include_str!(
        "svgs/pool_face_left.svg"
    )));

    // Navigation
    // let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    // add_onclick_listener!(
    //     [
    //         "toHallFaceUp",
    //         state.dispatch(set_current_room(Rooms::HallFaceUp))
    //     ],
    //     ["toLivingRoomFaceUp",state.dispatch(set_current_room(Rooms::LivingRoomFaceUp))]
    //     to_hall_face_up_effect,
    //     my_room_ref
    // );
    // use_effect(to_hall_face_up_effect);

    // let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    // add_onclick_listener!(["toLivingRoomFaceUp", to_living_room_face_up, my_room_ref,]);
    // use_effect(to_living_room_face_up);

    // let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    // add_onclick_listener!([
    //     "toStairsFaceUp",
    //     to_stairs_face_up,
    //     my_room_ref,
    //     state.dispatch(set_current_room(Rooms::StairsFaceUp))
    // ]);
    // use_effect(to_stairs_face_up);

    html! {
        <div id="my_room" ref={my_room_ref} class="rooms_CurrentRoom">
            {svg}
        </div>
    }
}
