use super::{add_onclick_listener, Rooms::*};

use crate::{
    items::{ItemFamily, ItemId::*},
    store::{
        actions,
        house::{place_knight, set_current_room},
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
        yew::Html::from_html_unchecked(yew::AttrValue::from(include_str!("svgs/chess_board.svg")));
    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    let is_knight_on_chess_board = state.house.is_knight_on_chess_board;

    // Set initial text
    use_effect_with_deps(
        {
            let state = state.clone();
            move |is_knight_on_chess_board: &bool| {
                state.clone().dispatch(actions![set_current_text(
                    if *is_knight_on_chess_board {
                        r#"Le cavalier semble parfaitement à sa place juste ici.""#
                    } else {
                        r#"Un jeu d'échec et une note : "Une seule pièce vous manque et tout est dépeuplé.""#
                    }
                )])
            }
        },
        is_knight_on_chess_board,
    );

    // Display or hide knight based on global state
    {
        use_effect_with_deps(
            {
                move |is_knight_on_chess_board: &bool| {
                    let horse_on_elem = gloo::utils::document()
                        .get_element_by_id("HorseOn")
                        .expect(&format!("HorseOn not found in svg"));
                    horse_on_elem
                        .set_attribute(
                            "class",
                            if *is_knight_on_chess_board {
                                "show"
                            } else {
                                "hidden"
                            },
                        )
                        .expect("Problem setting {id}'s attribute");
                }
            },
            is_knight_on_chess_board,
        );
    }

    // Place doudous on pile
    let on_board_click = {
        if state
            .items
            .family_selected
            .is_some_and(|family| family == ItemFamily::Knight)
        {
            let state = state.clone();
            Callback::from(move |_| {
                state.dispatch(actions![
                    unselect_family(),
                    remove_item_from_iventory(Knight),
                    place_knight()
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
            [("toFirstFloorFaceDown", move |_| state
                .dispatch(actions![set_current_room(FirstFloorFaceDown)]))],
            effect,
            room_ref
        );
        use_effect(effect);
    }

    html! {
        <div id="my_room" ref={room_ref} class="rooms_CurrentRoom" onclick={on_board_click}>
            {svg}
        </div>
    }
}
