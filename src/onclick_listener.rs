macro_rules! add {
    ([$($listeners: expr),*], $effect: ident, $state: ident, $room_ref: ident) => {
        #[allow(unused_variables)]  // For the case where listeners is empty
        let room_test = $room_ref.clone();
        let $effect = move || {
            #[allow(unused_mut)]  // For the case where listeners is empty
            let mut listeners: Vec::<Option<::gloo_events::EventListener>> = Vec::new();

            $(let (path_id, action) = $listeners;
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

                if let Some(element) = room_test.cast::<web_sys::HtmlElement>() {
                    let state_clone = $state.clone();
                    let on_custom_event =
                        Callback::from({
                            let action_clone = action.clone();
                            move |_| state_clone.dispatch(action_clone)
                        });
                    let listener = gloo::events::EventListener::new(&element, path_id, move |e| {
                        on_custom_event.emit(e.clone())
                    });

                    custom_listener = Some(listener);
                }

                listeners.push(custom_listener);
            )*
            move || drop(listeners)
        };
    };
}

pub(crate) use add;
