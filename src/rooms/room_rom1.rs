use crate::{
    items::ItemId::*,
    rooms::Rooms::*,
    store::{actions, items::find_object, narration::simple_description},
};

super::generate_room!(
    "svgs/room_rom1.svg",
    "Une chambre de mega bo goss",
    [FirstFloorFaceDown, BookPile],
    [
        state,
        simple_description!(
            "LibraryRom1",
            "Une autre page du journal. Pas utile pour me faire sortir d'ici mais intéressant."
        ),
        find_object!(
            state,
            "Jesus",
            NoteDoudous,
            "Jésus revient ? Avec une note sur les doudous",
            "Jésus n'est toujours pas là"
        )
    ]
);
