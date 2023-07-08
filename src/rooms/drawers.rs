use super::{add_onclick_listener, create_listener_from_callback};

use crate::{
    items::ItemId::*,
    rooms::Rooms::*,
    store::{
        actions, house::set_current_room, items::add_item_to_inventory, narration::set_current_text,
    },
    GlobalState,
};

use yew::prelude::*;

#[function_component(Room)]
pub(crate) fn html() -> Html {
    let room_ref = use_node_ref();
    let svg =
        yew::Html::from_html_unchecked(yew::AttrValue::from(include_str!("svgs/drawers.svg")));
    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    let useless_items_counter = use_state(|| 0);

    // Set initial text
    use_effect_with_deps(
        {
            let state = state.clone();
            move |_| {
                state.dispatch(actions![set_current_text(
                    "Des tiroirs clairement en bordel..."
                )]);
            }
        },
        state.house.current_room.clone(),
    );

    // Set onclick listeners for the search drawers
    {
        let room_ref = room_ref.clone();
        let state = state.clone();
        let text = USELESS_ITEMS_TEXTS.get(*useless_items_counter);
        use_effect(move || {
            let mut listeners: Vec<Option<::gloo_events::EventListener>> = Vec::new();
            let on_custom_event = Callback::from(move |_| {
                if state.items.items_found.contains(&Saw) {
                    state.dispatch(actions![set_current_text("Il n'y a plus rien d'utile")]);
                } else if let Some(text) = text {
                    useless_items_counter.set(*useless_items_counter + 1);
                    state.dispatch(actions![set_current_text(text)]);
                } else {
                    state.dispatch(actions![
                        set_current_text("Une mauvaise blague ... Ah, une scie !"),
                        add_item_to_inventory(Saw)
                    ]);
                }
            });
            create_listener_from_callback!(room_ref, listeners, "DrawersSearch1", on_custom_event);
            create_listener_from_callback!(room_ref, listeners, "DrawersSearch2", on_custom_event);
            create_listener_from_callback!(room_ref, listeners, "DrawersSearch3", on_custom_event);
            move || {
                drop(listeners);
            }
        })
    }

    // Navigation
    {
        let state = state.clone();
        add_onclick_listener!(
            [("toKitchenFaceDown", move |_| state
                .dispatch(actions![set_current_room(KitchenFaceDown)]))],
            effect,
            room_ref
        );
        use_effect(effect);
    }

    html! {
        <div id="my_room" ref={room_ref} class="rooms_CurrentRoom" >
            {svg}
        </div>
    }
}

const USELESS_ITEMS_TEXTS: [&'static str; 5] = [
    "On a tous un tiroir comme ça quelque-part",
    "Et c'est pas fini ",
    "Il reste deux trois bricoles au fond !",
    "C'est sans fin",
    "On dirait une blague",
];
