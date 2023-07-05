use crate::{
    rooms::Rooms::*,
    store::{actions, narration::set_current_text},
};

super::generate_room!(
    "svgs/room_mart1.svg",
    "Une chambre de bo goss",
    [FirstFloorFaceDown],
    [
        ("LibraryMart1", || actions![set_current_text(
            "Une autre page du journal. Pas utile pour me faire sortir d'ici mais intéressant."
        )]),
        ("Window", || actions![set_current_text(
            "Hey, on voit la voiture !"
        )])
    ]
);
