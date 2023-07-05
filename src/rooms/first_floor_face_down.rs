use crate::{
    rooms::Rooms::*,
    store::{actions, narration::set_current_text},
};

super::generate_room!(
    "svgs/first_floor_face_down.svg",
    "On dirait un grand espace de jeu mal rangé.",
    [RoomGui1, RoomMart1, RoomTiph1, RoomRom1, StairsFaceUp],
    [
        ("Shoes", || actions![set_current_text(
            "Un drôle de meuble à chaussures."
        )]),
        ("Closet", || actions![set_current_text(
            "Un placard. Tiens, encore une de ces bandelettes."
        )]),
        ("Box", || actions![set_current_text(
            r#"Un jeu d'échec et une note : "Une seule pièce vous manque et tout est dépeuplé.""#
        )])
    ],
);
