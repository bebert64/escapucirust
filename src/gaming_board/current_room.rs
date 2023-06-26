use crate::{GameStatus, GlobalState, StateAction};

use crate::onclick_listener::add;

use yew::prelude::*;

#[function_component(Component)]
pub(crate) fn html() -> Html {
    let my_room_ref = use_node_ref();
    let parsed =
        yew::Html::from_html_unchecked(yew::AttrValue::from(include_str!("hall_face_up.svg")));

    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");

    add!(
        "HallFrame",
        hall_frame_create_event,
        hall_frame_handle_event,
        my_room_ref,
        state.dispatch(StateAction::Status(GameStatus::Starting))
    );
    use_effect(hall_frame_create_event);
    use_effect(hall_frame_handle_event);

    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    add!(
        "TreeOfHat",
        tree_of_hat_create_event,
        tree_of_hat_handle_event,
        my_room_ref,
        state.dispatch(StateAction::Status(GameStatus::Intro))
    );
    use_effect(tree_of_hat_create_event);
    use_effect(tree_of_hat_handle_event);

    html! {
        <div id="my_room" ref={my_room_ref} class="rooms_CurrentRoom">
            {parsed}
       // <div class="rooms_BlackVeil"></div>
        </div>
    }
}
