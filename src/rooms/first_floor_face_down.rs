use crate::{
    items::ItemId::*,
    rooms::Rooms::*,
    store::{actions, items::add_item_to_inventory, narration::set_current_text},
};

super::generate_room!(
    "svgs/first_floor_face_down.svg",
    "On dirait un grand espace de jeu mal rangé.",
    [RoomGui1, RoomMart1, RoomTiph1, RoomRom1, StairsFaceUp],
    [
        state,
        ("Shoes", || actions![set_current_text(
            "Un drôle de meuble à chaussures."
        )]),
        (
            "Closet",
            if !state.items.items_found.contains(&Strip3) {
                || {
                    actions![
                        set_current_text("Un placard. Tiens, encore une de ces bandelettes."),
                        add_item_to_inventory(Strip3)
                    ]
                }
            } else {
                || actions![set_current_text("Un placard")]
            }
        ),
        ("Box", || actions![set_current_text(
            r#"Un jeu d'échec et une note : "Une seule pièce vous manque et tout est dépeuplé.""#
        )])
    ],
);
