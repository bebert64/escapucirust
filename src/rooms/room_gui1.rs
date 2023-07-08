use crate::{rooms::Rooms::*, store::narration::simple_description};

super::generate_room!(
    "svgs/room_gui1.svg",
    "La chambre de Gui1",
    [FirstFloorFaceDown],
    [
        simple_description!(
            "LibraryGui",
            "Capitaine Tsubasa, Tom-Tom et Nana, et d'autres bouquins pour enfant"
        ),
        simple_description!(
            "DrawersGui",
            "Des affaires de foot et des t-shirts beaucoup trop grand pour un enfant"
        ),
        simple_description!(
            "Hatch",
            "C'est déjà bien assez effrayant comme ça... Pourquoi aller voir le grenier ?"
        ),
    ]
);
