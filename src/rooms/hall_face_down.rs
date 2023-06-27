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
        "svgs/hall_face_down.svg"
    )));

    // // Navigation
    // let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    // add_onclick_listener!([
    //     "toPoolFaceLeft",
    //     to_pool_face_left,
    //     my_room_ref,
    //     state.dispatch(set_current_room(Rooms::PoolFaceLeft))
    // ]);
    // use_effect(to_pool_face_left);

    // Init
    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    use_effect_with_deps(
        {
            let state = state.clone();
            move |_| state.clone().dispatch(set_current_text("Enter down"))
        },
        state.house_state.current_room.clone(),
    );

    html! {
        <div id="my_room" ref={my_room_ref} class="rooms_CurrentRoom">
            {svg}
        </div>
    }
}
