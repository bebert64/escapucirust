use crate::{
    rooms::Rooms::*,
    store::{
        actions,
        house::{force_door_mart1, set_current_room},
        narration::{set_current_text, simple_description},
    },
};

super::generate_room!(
    "svgs/room_mart1.svg",
    "Une chambre de bo goss",
    [],
    [
        state,
        simple_description!(
            "LibraryMart1",
            "Une autre page du journal. Pas utile pour me faire sortir d'ici mais intéressant."
        ),
        simple_description!("Window", "Hey, on voit la voiture !"),
        (
            "toFirstFloorFaceDown",
            if state.house.is_door_mart1_blocked {
                || {
                    actions![set_current_text("La dorte semble bloquée. Il va falloir donner un bon gros coup d'épaule"), force_door_mart1()]
                }
            } else {
                || actions![set_current_room(FirstFloorFaceDown)]
            }
        )
    ]
);
