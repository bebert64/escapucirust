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
                super::add_on_click_listener!(
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
                super::add_on_click_listener!(
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

pub(crate) use generate_room;
