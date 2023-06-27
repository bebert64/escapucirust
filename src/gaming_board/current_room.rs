use crate::{
    add_onclick_listener,
    store::{game_status::display_start_menu, house_state::toggle_light},
    GlobalState,
};

use yew::prelude::*;

#[function_component(Component)]
pub(crate) fn html() -> Html {
    let my_room_ref = use_node_ref();
    let parsed =
        yew::Html::from_html_unchecked(yew::AttrValue::from(include_str!("hall_face_up.svg")));

    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");

    add_onclick_listener!(
        "HallFrame",
        hall_frame_effect,
        my_room_ref,
        state.dispatch(display_start_menu())
    );
    use_effect(hall_frame_effect);

    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    add_onclick_listener!(
        "TreeOfHat",
        tree_of_hat_effect,
        my_room_ref,
        state.dispatch(toggle_light())
    );
    use_effect(tree_of_hat_effect);

    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    html! {
        <div id="my_room" ref={my_room_ref} class="rooms_CurrentRoom">
            {parsed}
       {if state.house_state.is_light_on {html!{<div class="rooms_BlackVeil"></div>}} else {html!{<></>}}}
        </div>
    }
}
