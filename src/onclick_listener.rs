macro_rules! add {
    ($path_id: expr, $create: ident, $handle: ident, $room_ref: ident, $callback: expr) => {
        let $create = move || {
            let element = gloo::utils::document()
                .get_element_by_id($path_id)
                .expect("$path_id not found in hall.svg");
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
        };

        let room_test = $room_ref.clone();
        let $handle = move || {
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
