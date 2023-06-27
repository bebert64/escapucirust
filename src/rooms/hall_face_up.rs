use crate::{
    add_onclick_listener,
    rooms::Rooms,
    store::{
        game_status::display_start_menu,
        house_state::{set_current_room, toggle_light},
        narration::set_current_text,
    },
    GlobalState,
};

use yew::prelude::*;

#[function_component(Component)]
pub(crate) fn html() -> Html {
    let my_room_ref = use_node_ref();
    let svg =
        yew::Html::from_html_unchecked(yew::AttrValue::from(include_str!("svgs/hall_face_up.svg")));

    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    add_onclick_listener!(
        "HallFrame",
        hall_frame_effect,
        my_room_ref,
        state.dispatch(display_start_menu())
    );
    use_effect(hall_frame_effect);

    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    add_onclick_listener!("TreeOfHat", tree_of_hat_effect, my_room_ref, {
        state.dispatch(toggle_light());
        state.dispatch(set_current_text("Lights toggled"));
    });
    use_effect(tree_of_hat_effect);

    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    add_onclick_listener!(
        "toHallFaceDown",
        to_hall_face_down_effect,
        my_room_ref,
        state.dispatch(set_current_room(Rooms::HallFaceDown))
    );
    use_effect(to_hall_face_down_effect);

    html! {
        <div id="my_room" ref={my_room_ref} class="rooms_CurrentRoom">
            {svg}
        </div>
    }
}
