use crate::rooms::Rooms::*;

super::generate_room!(
    "svgs/dining_room_face_up.svg",
    "Enter dining_room_face_up",
    [("zz", crate::store::house_state::toggle_light)],
    [LivingRoomFaceRight, KitchenFaceDown]
);
