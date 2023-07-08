use crate::{rooms::Rooms::*, store::narration::simple_description};

super::generate_room!(
    "svgs/room_tiph1.svg",
    "Une chambre de gonzesse, mais sans le rose",
    [FirstFloorFaceDown],
    [
        simple_description!(
            "LibraryTiph1",
            "Une autre page du journal. Pas utile pour me faire sortir d'ici mais intéressant."
        ),
        simple_description!(
            "PieceOnFloor",
            "Un cavalier du jeu d'échec. Que fait-il ici ?"
        )
    ]
);
