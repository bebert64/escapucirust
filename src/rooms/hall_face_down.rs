use crate::rooms::Rooms::*;

super::generate_room!(
    "svgs/hall_face_down.svg",
    "Enter down",
    [],
    [],
    [HallFaceUp, PoolFaceLeft]
);
