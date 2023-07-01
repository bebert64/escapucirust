use crate::rooms::Rooms::*;

super::generate_room!(
    "svgs/kitchen_face_left.svg",
    "Enter kitchen_face_left",
    [],
    [
        KitchenFaceDown,
        Frigogidaire,
        DiningRoomFaceUp,
        StairsFaceUp
    ]
);
