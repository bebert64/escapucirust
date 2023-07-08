use super::{add_onclick_listener, Rooms::*};

use crate::{
    items::{ItemFamily, ItemId::*},
    store::{
        actions,
        house::{place_doudou, set_current_room},
        items::{remove_item_from_iventory, unselect_family},
        narration::set_current_text,
    },
    GlobalState,
};

use yew::prelude::*;

#[function_component(Room)]
pub(crate) fn html() -> Html {
    let room_ref = use_node_ref();
    let svg =
        yew::Html::from_html_unchecked(yew::AttrValue::from(include_str!("svgs/book_pile.svg")));
    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");

    // Set initial text
    use_effect_with_deps(
        {
            let state = state.clone();
            move |_| {
                state.clone().dispatch(actions![set_current_text(
                    "Un tas haut et un tabac. Quelle bande de comique...",
                )])
            }
        },
        state.house.current_room.clone(),
    );

    // Display and hide doudous based on global state
    {
        let doudous_placed = state
            .house
            .doudous_placed
            .iter()
            .map(|doudou_id| format!("{doudou_id:?}"))
            .collect::<Vec<_>>();
        fn set_class(doudou_element_id: &str, class: &str) {
            let doudou_elem = gloo::utils::document()
                .get_element_by_id(doudou_element_id)
                .expect(&format!("{doudou_element_id} not found in svg"));
            doudou_elem
                .set_attribute("class", class)
                .expect("Problem setting {id}'s attribute");
        }
        use_effect_with_deps(
            {
                move |doudous_placed: &Vec<String>| {
                    for doudou_index in 1..5 {
                        let doudou_id = format!("Doudou{doudou_index}");
                        if doudous_placed.contains(&doudou_id) {
                            set_class(&doudou_id, "show");
                        } else {
                            set_class(&doudou_id, "hidden");
                        }
                    }
                }
            },
            doudous_placed.clone(),
        );
    }

    // Place doudous on pile
    let on_pile_click = {
        let doudou_to_place = match state.items.family_selected {
            Some(ItemFamily::Doudou1) => Some(Doudou1),
            Some(ItemFamily::Doudou2) => Some(Doudou2),
            Some(ItemFamily::Doudou3) => Some(Doudou3),
            Some(ItemFamily::Doudou4) => Some(Doudou4),
            _ => None,
        };
        if let Some(doudou) = doudou_to_place {
            let state = state.clone();
            Callback::from(move |_| {
                state.dispatch(actions![
                    unselect_family(),
                    remove_item_from_iventory(doudou),
                    place_doudou(doudou)
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
            [("toRoomRom1", move |_| state
                .dispatch(actions![set_current_room(RoomRom1)]))],
            effect,
            room_ref
        );
        use_effect(effect);
    }

    html! {
        <div id="my_room" ref={room_ref} class="rooms_CurrentRoom" onclick={on_pile_click}>
            {svg}
        </div>
    }
}
