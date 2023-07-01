use crate::{
    rooms::Rooms::*,
    store::{actions, narration::set_current_text},
};

super::generate_room!(
    "svgs/dining_room_face_up.svg",
    "Enter dining_room_face_up",
    [LivingRoomFaceRight, KitchenFaceDown],
    [("Door", || {
        actions![
            set_current_text("Une porte vers l'extérieur. Dommage que les devs aient eu la flemme de programmer un extérieur.")
        ]
    })],
);
