use super::{add_onclick_listener, Rooms::*};

use crate::{
    items::ItemFamily::*,
    store::{
        actions,
        house::{place_strip, set_current_room},
        items::{remove_item_from_iventory, unselect_family},
        narration::set_current_text,
    },
    GlobalState,
};

use yew::prelude::*;

#[function_component(Room)]
pub(crate) fn html() -> Html {
    let room_ref = use_node_ref();
    let svg = yew::Html::from_html_unchecked(yew::AttrValue::from(include_str!(
        "svgs/strips_podium.svg"
    )));
    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");

    // Set initial text
    use_effect_with_deps(
        {
            let state = state.clone();
            move |_| {
                state.clone().dispatch(actions![set_current_text(
                    "Un étrange podium à bandelettes, quoi qu'un podium à bandelettes puisse être...",
                )])
            }
        },
        state.house.current_room.clone(),
    );

    // Display and hide strips based on global state
    {
        let strips_placed = state
            .house
            .strips_placed
            .iter()
            .map(|strip_id| format!("{strip_id:?}"))
            .collect::<Vec<_>>();
        fn set_class(strip_element_id: &str, class: &str) {
            let strip_element = gloo::utils::document()
                .get_element_by_id(strip_element_id)
                .expect(&format!("{strip_element_id} not found in svg"));
            strip_element
                .set_attribute("class", class)
                .expect(&format!("Problem setting {strip_element_id}'s attribute"));
        }
        use_effect_with_deps(
            {
                move |strips_placed: &Vec<String>| {
                    for strip_index in 1..8 {
                        let strip_id = format!("Strip{strip_index}");
                        if strips_placed.contains(&strip_id) {
                            set_class(&strip_id, "show");
                        } else {
                            set_class(&strip_id, "hidden");
                        }
                    }
                }
            },
            strips_placed.clone(),
        );
    }

    // Place strips on podium
    let on_podium_click = {
        if state
            .items
            .family_selected
            .is_some_and(|family| family == Strip)
        {
            let strip_to_place = *state
                .items
                .inventory()
                .get(&Strip)
                .expect("If Strip is selected, it should be in the inventory")
                .first()
                .expect("No empty vec in inventory");
            let state = state.clone();
            Callback::from(move |_| {
                state.dispatch(actions![
                    unselect_family(),
                    remove_item_from_iventory(strip_to_place),
                    place_strip(strip_to_place)
                ]);
            })
        } else {
            Callback::default()
        }
    };

    // Navigation
    {
        let state = state.clone();
        add_onclick_listener!(
            [("toRoomTiph1", move |_| state
                .dispatch(actions![set_current_room(RoomTiph1)]))],
            effect,
            room_ref
        );
        use_effect(effect);
    }

    html! {
        <div id="my_room" ref={room_ref} class="rooms_CurrentRoom" onclick={on_podium_click}>
            {svg}
        </div>
    }
}
