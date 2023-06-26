use crate::{GameStatus, GlobalState, StateAction};

use yew::prelude::*;

#[function_component(Component)]
pub(crate) fn html() -> Html {
    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    let parsed =
        yew::Html::from_html_unchecked(yew::AttrValue::from(include_str!("hall_face_up.svg")));
    let my_room_ref = use_node_ref();

    use_effect(move || {
        let hall_frame = gloo::utils::document()
            .get_element_by_id("HallFrame")
            .expect("HallFrame not found in hall.svg");
        hall_frame
            .add_event_listener_with_callback(
                "click",
                &js_sys::Function::new_no_args(
                    r#"
                    var event = new CustomEvent("hall_frame", {"bubbles": true});
                    my_room.dispatchEvent(event);
                "#,
                ),
            )
            .unwrap();
    });
    use_effect_with_deps(
        {
            let state = state.clone();
            let div_node_ref = my_room_ref.clone();

            move |_| {
                let mut custom_listener = None;

                if let Some(element) = div_node_ref.cast::<web_sys::HtmlElement>() {
                    // Create your Callback as you normally would
                    let on_custom_event = Callback::from(move |_| {
                        state.dispatch(StateAction::Status(GameStatus::Intro))
                    });

                    // Create a Closure from a Box<dyn Fn> - this has to be 'static
                    let listener =
                        gloo::events::EventListener::new(&element, "hall_frame", move |e| {
                            on_custom_event.emit(e.clone())
                        });

                    custom_listener = Some(listener);
                }

                move || drop(custom_listener)
            }
        },
        my_room_ref.clone(),
    );

    use_effect(move || {
        let tree_of_hat = gloo::utils::document()
            .get_element_by_id("TreeOfHat")
            .expect("TreeOfHat not found in hall.svg");
        tree_of_hat
            .add_event_listener_with_callback(
                "click",
                &js_sys::Function::new_no_args(
                    r#"
                    var event = new CustomEvent("tree_of_hat", {"bubbles": true});
                    my_room.dispatchEvent(event);
                "#,
                ),
            )
            .unwrap();
    });
    use_effect_with_deps(
        {
            let state = state.clone();
            let div_node_ref = my_room_ref.clone();

            move |_| {
                let mut custom_listener = None;

                if let Some(element) = div_node_ref.cast::<web_sys::HtmlElement>() {
                    // Create your Callback as you normally would
                    let on_custom_event = Callback::from(move |_| {
                        state.dispatch(StateAction::Status(GameStatus::Starting))
                    });

                    // Create a Closure from a Box<dyn Fn> - this has to be 'static
                    let listener =
                        gloo::events::EventListener::new(&element, "tree_of_hat", move |e| {
                            on_custom_event.emit(e.clone())
                        });

                    custom_listener = Some(listener);
                }

                move || drop(custom_listener)
            }
        },
        my_room_ref.clone(),
    );

    html! {
        <div id="my_room" ref={my_room_ref} class="rooms_CurrentRoom">
            {parsed}
       // <div class="rooms_BlackVeil"></div>
        </div>
    }
}
