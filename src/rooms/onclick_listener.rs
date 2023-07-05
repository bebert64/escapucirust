macro_rules! add {
    ([$($listener: expr),+], $effect: ident, $room_ref: ident) => {
        let room_ref = $room_ref.clone();
        let $effect = move || {
            let mut listeners: Vec::<Option<::gloo_events::EventListener>> = Vec::new();

            $(let (path_id, action) = $listener;
                crate::rooms::onclick_listener::create_listener!(room_ref, listeners, path_id, action);
            )+
            move || drop(listeners)
        };
    };
}

macro_rules! create_listener {
    ($room_ref: ident, $listeners: ident, $path_id: expr, $action: expr) => {
        let on_custom_event = Callback::from($action);

        crate::rooms::onclick_listener::create_listener_from_callback!(
            $room_ref,
            $listeners,
            $path_id,
            on_custom_event
        )
    };
}

macro_rules! create_listener_from_callback {
    ($room_ref: ident, $listeners: ident, $path_id: expr, $callback: ident) => {
        let element = gloo::utils::document()
            .get_element_by_id(&$path_id)
            .expect(&format!("{} not found in svg", &$path_id));
        if !element
            .get_attribute("listener")
            .is_some_and(|attr| attr == "set")
        {
            element
                .add_event_listener_with_callback(
                    "click",
                    &::js_sys::Function::new_no_args(
                        format!(
                            r#"var event = new CustomEvent("{}", {{"bubbles": true}});
                            my_room.dispatchEvent(event);"#,
                            $path_id
                        )
                        .as_str(),
                    ),
                )
                .expect(&format!("Issue adding the event listener for {}", $path_id));
            element.set_attribute("listener", "set").expect(&format!(
                "Issue setting the attribute 'listener' for {}",
                $path_id
            ));
        }
        let custom_listener;
        let callback = $callback.clone();

        if let Some(element) = $room_ref.cast::<web_sys::HtmlElement>() {
            let listener = gloo::events::EventListener::new(&element, $path_id, move |e| {
                callback.emit(e.clone())
            });

            custom_listener = Some(listener);

            $listeners.push(custom_listener);
        }
    };
}

pub(crate) use {add, create_listener, create_listener_from_callback};
