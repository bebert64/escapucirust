macro_rules! generate_room {
    ($svg: expr, $init_text: expr, [$($listener: expr),*], [$($room: expr),* ])=> {
        use crate::{GlobalState, store::GlobalStateActions};

        use yew::prelude::*;

        #[function_component(Room)]
        pub(crate) fn html() -> Html {
            let room_ref = use_node_ref();
            let svg = yew::Html::from_html_unchecked(yew::AttrValue::from(include_str!($svg)));

            // Set initial text
            let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
            use_effect_with_deps(
                {
                    let state = state.clone();
                    move |_| {
                        state
                            .clone()
                            .dispatch(
                                GlobalStateActions{
                                    actions: vec![crate::store::narration::set_current_text($init_text)]
                                }
                            )
                    }
                },
                state.house.current_room.clone(),
            );

            // Set listeners for multiple or complex actions
            $({
                let room_ref = room_ref.clone();
                let (path_id, create_actions) = $listener;
                let state = state.clone();
                let effect = move || {
                    let element = gloo::utils::document()
                        .get_element_by_id(&path_id)
                        .expect(&format!("{} not found in svg", &path_id));
                    if !element
                        .get_attribute("listener")
                        .is_some_and(|attr| attr == "set")
                    {
                        element
                            .add_event_listener_with_callback(
                                "click",
                                &::js_sys::Function::new_no_args(
                                    format!(
                                        r#"
                            var event = new CustomEvent("{}", {{"bubbles": true}});
                            my_room.dispatchEvent(event);
                            console.log("fired");
                        "#,
                                        path_id
                                    )
                                    .as_str(),
                                ),
                            )
                            .unwrap();
                        element.set_attribute("listener", "set").unwrap();
                    }
                    let mut custom_listener = None;

                    if let Some(element) = room_ref.cast::<web_sys::HtmlElement>() {
                        let on_custom_event =
                            Callback::from({
                                let state_clone = state.clone();
                                move |_| state_clone.dispatch(create_actions())
                            });
                        let listener = gloo::events::EventListener::new(&element, path_id, move |e| {
                            on_custom_event.emit(e.clone())
                        });

                        custom_listener = Some(listener);
                    }

                    move || drop(custom_listener)
                };
                use_effect(effect);
            })*

            // Set listeners for navigation between rooms
            $({
                let room_ref = room_ref.clone();
                let state_clone = state.clone();
                let create_actions = || GlobalStateActions {
                    actions: vec![crate::store::house::set_current_room($room)]
                };
                let path_id = format!("to{}", stringify!($room));
                let effect = move || {
                    let element = gloo::utils::document()
                        .get_element_by_id(&path_id)
                        .expect(&format!("{} not found in svg", &path_id));
                    if !element
                        .get_attribute("listener")
                        .is_some_and(|attr| attr == "set")
                    {
                        element
                            .add_event_listener_with_callback(
                                "click",
                                &::js_sys::Function::new_no_args(
                                    format!(
                                        r#"
                            var event = new CustomEvent("{}", {{"bubbles": true}});
                            my_room.dispatchEvent(event);
                            console.log("fired");
                        "#,
                                        path_id
                                    )
                                    .as_str(),
                                ),
                            )
                            .unwrap();
                        element.set_attribute("listener", "set").unwrap();
                    }
                    let mut custom_listener = None;

                    if let Some(element) = room_ref.cast::<web_sys::HtmlElement>() {
                        let on_custom_event =
                            Callback::from({
                                move |_| state_clone.dispatch(create_actions())
                            });
                        let listener = gloo::events::EventListener::new(&element, path_id, move |e| {
                            on_custom_event.emit(e.clone())
                        });

                        custom_listener = Some(listener);
                    }

                    move || drop(custom_listener)
                };
                use_effect(effect);
            })*


            html! {
                <div id="my_room" ref={room_ref} class="rooms_CurrentRoom">
                    {svg}
                </div>
            }
        }
    };
}

pub(crate) use generate_room;
