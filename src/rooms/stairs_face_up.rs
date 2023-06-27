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
        "svgs/stairs_face_up.svg"
    )));

    // let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    // add_onclick_listener!(
    //     "toHallFaceUp",
    //     to_hall_face_up_effect,
    //     my_room_ref,
    //     state.dispatch(set_current_room(Rooms::HallFaceUp))
    // );
    // use_effect(to_hall_face_up_effect);

    html! {
        <div id="my_room" ref={my_room_ref} class="rooms_CurrentRoom">
            {svg}
        </div>
    }
}
