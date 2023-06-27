macro_rules! add {
    ($path_id: expr, $effect: ident, $room_ref: ident, $callback: expr) => {
        let room_test = $room_ref.clone();
        let $effect = move || {
            let element = gloo::utils::document()
                .get_element_by_id($path_id)
                .expect("$path_id not found in hall.svg");
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
                                $path_id
                            )
                            .as_str(),
                        ),
                    )
                    .unwrap();
                element.set_attribute("listener", "set").unwrap();
            }
            let mut custom_listener = None;

            if let Some(element) = room_test.cast::<web_sys::HtmlElement>() {
                let on_custom_event = Callback::from(move |_| $callback);
                let listener = gloo::events::EventListener::new(&element, $path_id, move |e| {
                    on_custom_event.emit(e.clone())
                });

                custom_listener = Some(listener);
            }

            move || drop(custom_listener)
        };
    };
}

pub(crate) use add;
