use crate::{
    items::ItemId::*,
    rooms::Rooms::*,
    store::{
        actions,
        house::set_current_room,
        items::find_object,
        narration::{set_current_text, simple_description},
    },
};

super::generate_room!(
    "svgs/room_rom1.svg",
    "Une chambre de mega bo goss",
    [FirstFloorFaceDown],
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
        ),
        (
            "toBookPile",
            if !state.house.is_door_tiph1_open {
                || actions![set_current_room(BookPile)]
            } else {
                || {
                    actions![set_current_text(
                        "Juste des doudous et des livres, pas hyper intéressant..."
                    )]
                }
            }
        )
    ]
);
