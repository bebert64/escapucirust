use crate::{
    items::{ItemFamily, ItemId::*},
    rooms::{display_element_if, Rooms::*},
    store::{
        actions,
        house::{place_board_on_hole, set_current_room},
        items::remove_item_from_iventory,
        narration::set_current_text,
    },
};

super::generate_room!(
    "svgs/stairs_face_up.svg",
    "",
    [PoolFaceLeft, KitchenFaceLeft],
    [
        state,
        ("Drawing", || {
            actions![set_current_text(
                r#"Si on tourne un peu la tête, on dirait un portrait... Hmm...
                Non, je n'ai pas la moindre idée de ce que c'est."#
            )]
        }),
        ("HorrorPainting", || {
            actions![set_current_text(
                "Ils sortent du mur quand même, les trucs, là..!"
            )]
        }),
        ("Hole", {
            if state
                .items
                .family_selected
                .is_some_and(|family| family == ItemFamily::Board)
            {
                || actions![place_board_on_hole(), remove_item_from_iventory(Board)]
            } else {
                || actions![]
            }
        }),
        ("toFirstFloorFaceDown", {
            if state.house.is_board_on_hole {
                || actions![set_current_room(FirstFloorFaceDown)]
            } else {
                || actions![set_current_text("Nope, le trou est beaucoup trop profond.")]
            }
        })
    ],
    {
        let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
        let is_board_on_hole = state.house.is_board_on_hole;
        {
            let state = state.clone();
            use_effect_with_deps(
                move |_: &bool| {
                    state.dispatch(actions![set_current_text(
                        if !state.house.is_board_on_hole {
                            r#"Wow, ce trou est impossible à franchir !
                    "Il semble aller direct en Enfer."#
                        } else {
                            "Le trou semble beaucoup plus franchissable maintenant"
                        }
                    )]);
                },
                is_board_on_hole,
            );
        }
        use_effect_with_deps(
            display_element_if!(is_board_on_hole, "Board"),
            is_board_on_hole,
        );
    }
);
