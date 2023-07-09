pub(crate) mod book_pile;
pub(crate) mod chess_board;
pub(crate) mod dining_room_face_up;
pub(crate) mod door_gui1;
pub(crate) mod door_mart1;
pub(crate) mod door_rom1;
pub(crate) mod door_tiph1;
pub(crate) mod drawers;
pub(crate) mod electrical_panel;
pub(crate) mod first_floor_face_down;
pub(crate) mod frigogidaire;
pub(crate) mod hall_face_down;
pub(crate) mod hall_face_up;
pub(crate) mod kitchen_face_down;
pub(crate) mod kitchen_face_left;
pub(crate) mod living_room_face_right;
pub(crate) mod living_room_face_up;
pub(crate) mod pool_face_down;
pub(crate) mod pool_face_left;
pub(crate) mod room_gui1;
pub(crate) mod room_mart1;
pub(crate) mod room_rom1;
pub(crate) mod room_tiph1;
pub(crate) mod stairs_face_up;
pub(crate) mod strips_podium;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Rooms {
    DiningRoomFaceUp,
    Drawers,
    ElectricalPanel,
    FirstFloorFaceDown,
    Frigogidaire,
    HallFaceDown,
    HallFaceUp,
    KitchenFaceDown,
    KitchenFaceLeft,
    LivingRoomFaceRight,
    LivingRoomFaceUp,
    PoolFaceDown,
    PoolFaceLeft,
    RoomGui1,
    RoomMart1,
    RoomTiph1,
    RoomRom1,
    BookPile,
    StairsFaceUp,
    DoorGui1,
    DoorMart1,
    DoorRom1,
    DoorTiph1,
    StripsPodium,
    ChessBoard,
}

macro_rules! generate_room {
    ($svg: expr, $init_text: expr, [$($room: expr),* ], [$state: ident, $($listener: expr),* $(,)?], $use_effect: expr) => {
        use crate::{GlobalState, store::GlobalStateActions};

        use yew::prelude::*;

        #[function_component(Room)]
        pub(crate) fn html() -> Html {
            let room_ref = use_node_ref();
            let svg = yew::Html::from_html_unchecked(yew::AttrValue::from(include_str!($svg)));

            // Set initial text
            let $state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
            use_effect_with_deps(
                {
                    let state = $state.clone();
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
                $state.house.current_room.clone(),
            );

            // Set listeners for navigation between rooms
            $({
                let room_ref = room_ref.clone();
                let state = $state.clone();
                let path_id = format!("to{}", stringify!($room));
                super::add_onclick_listener!(
                    [(path_id, move |_| state.dispatch(GlobalStateActions {
                        actions: vec![crate::store::house::set_current_room($room)]
                    }))],
                    effect,
                    room_ref
                );
                use_effect(effect);
            })*

            // Set listeners for multiple or complex actions
            $({
                let room_ref = room_ref.clone();
                let (path_id, create_actions) = $listener;
                let state = $state.clone();
                super::add_onclick_listener!(
                    [(path_id, move |_| state.dispatch(create_actions()))],
                    effect,
                    room_ref
                );
                use_effect(effect);
            })*

            $use_effect;

            html! {
                <div id="my_room" ref={room_ref} class="rooms_CurrentRoom">
                    {svg}
                </div>
            }
        }
    };

    ($svg: expr, $init_text: expr, [$($room: expr),* ], [$state: ident, $($listener: expr),* $(,)?] $(,)?) => {
        crate::rooms::generate_room!($svg, $init_text, [$($room),*], [$state, $($listener),*], ());
    };

    ($svg: expr, $init_text: expr, [$($room: expr),* ], [$($listener: expr),* $(,)?], $use_effect: expr) => {
        crate::rooms::generate_room!($svg, $init_text, [$($room),*], [state, $($listener),*], $use_effect);
    };

    ($svg: expr, $init_text: expr, [$($room: expr),* ], [$($listener: expr),* $(,)?] $(,)?) => {
        crate::rooms::generate_room!($svg, $init_text, [$($room),*], [state, $($listener),*], ());
    };
}

macro_rules! add_onclick_listener {
    ([$($listener: expr),+], $effect: ident, $room_ref: ident) => {
        let room_ref = $room_ref.clone();
        let $effect = move || {
            let mut listeners: Vec::<Option<::gloo_events::EventListener>> = Vec::new();

            $(let (path_id, action) = $listener;
                crate::rooms::create_listener!(room_ref, listeners, path_id, action);
            )+
            move || drop(listeners)
        };
    };
}

macro_rules! create_listener {
    ($room_ref: ident, $listeners: ident, $path_id: expr, $action: expr) => {
        let on_custom_event = Callback::from($action);

        crate::rooms::create_listener_from_callback!(
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

macro_rules! display_element_if {
    ($condition: ident, $element_id: expr) => {
        if $condition {
            |_: &bool| {
                let table_cut = gloo::utils::document()
                    .get_element_by_id($element_id)
                    .expect(&format!("{} not found in svg", $element_id));
                table_cut
                    .set_attribute("class", "show")
                    .expect(&format!("{} not found in svg", $element_id));
            }
        } else {
            |_: &bool| {
                let table_cut = gloo::utils::document()
                    .get_element_by_id($element_id)
                    .expect(&format!("{} not found in svg", $element_id));
                table_cut
                    .set_attribute("class", "hidden")
                    .expect(&format!("{} not found in svg", $element_id));
            }
        }
    };
}

pub(crate) use {
    add_onclick_listener, create_listener, create_listener_from_callback, display_element_if,
    generate_room,
};
