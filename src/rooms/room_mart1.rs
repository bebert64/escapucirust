use crate::{rooms::Rooms::*, store::narration::simple_description};

super::generate_room!(
    "svgs/room_mart1.svg",
    "Une chambre de bo goss",
    [FirstFloorFaceDown],
    [
        simple_description!(
            "LibraryMart1",
            "Une autre page du journal. Pas utile pour me faire sortir d'ici mais intéressant."
        ),
        simple_description!("Window", "Hey, on voit la voiture !")
    ]
);
