use crate::{
    rooms::Rooms::*,
    store::{actions, narration::set_current_text},
};

super::generate_room!(
    "svgs/room_rom1.svg",
    "Une chambre de mega bo goss",
    [FirstFloorFaceDown],
    [
        ("LibraryRom1", || actions![set_current_text(
            "Une autre page du journal. Pas utile pour me faire sortir d'ici mais intéressant."
        )]),
        ("Book", || actions![set_current_text(
            "Un tas haut et un tabac. Quelle bande de comique..."
        )])
    ]
);
