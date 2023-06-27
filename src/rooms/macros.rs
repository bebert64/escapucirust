macro_rules! generate_room {
    ($svg: expr, $init_text: expr, [$($listeners: expr),+], [$(($path: expr, $room: expr)),+ ])=> {
        use crate::{GlobalState};

        use yew::prelude::*;

        #[function_component(Component)]
        pub(crate) fn html() -> Html {
            let my_room_ref = use_node_ref();
            let svg = yew::Html::from_html_unchecked(yew::AttrValue::from(include_str!($svg)));

            // Init
            let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
            use_effect_with_deps(
                {
                    let state = state.clone();
                    move |_| {
                        state
                            .clone()
                            .dispatch(crate::store::narration::set_current_text($init_text))
                    }
                },
                state.house_state.current_room.clone(),
            );

            let state_clone = state.clone();
            crate::add_onclick_listener!([$($listeners),+], effect, state_clone, my_room_ref);
            use_effect(effect);

            let state_clone = state.clone();
            crate::add_onclick_listener!([$(($path, || crate::store::house_state::set_current_room($room))),+], effect, state_clone, my_room_ref);
            use_effect(effect);

            html! {
                <div id="my_room" ref={my_room_ref} class="rooms_CurrentRoom">
                    {svg}
                </div>
            }
        }
    };
}

pub(crate) use generate_room;
