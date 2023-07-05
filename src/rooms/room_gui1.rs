use crate::{
    rooms::Rooms::*,
    store::{actions, narration::set_current_text},
};

super::generate_room!(
    "svgs/room_gui1.svg",
    "La chambre de Gui1",
    [FirstFloorFaceDown],
    [
        ("LibraryGui", || actions!(set_current_text(
            "Capitaine Tsubasa, Tom-Tom et Nana, et d'autres bouquins pour enfant"
        ))),
        ("DrawersGui", || actions!(set_current_text(
            "Des affaires de foot et des t-shirts beaucoup trop grand pour un enfant"
        ))),
        ("Hatch", || actions!(set_current_text(
            "C'est déjà bien assez effrayant comme ça... Pourquoi aller voir le grenier ?"
        ))),
    ]
);
