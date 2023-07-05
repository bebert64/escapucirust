use crate::{
    rooms::Rooms::*,
    store::{actions, narration::set_current_text},
};

super::generate_room!(
    "svgs/room_tiph1.svg",
    "Une chambre de gonzesse, mais sans le rose",
    [FirstFloorFaceDown],
    [
        ("LibraryTiph1", || actions![set_current_text(
            "Une autre page du journal. Pas utile pour me faire sortir d'ici mais intéressant."
        )]),
        ("PieceOnFloor", || actions![set_current_text(
            "Un cavalier du jeu d'échec. Que fait-il ici ?"
        )])
    ]
);
