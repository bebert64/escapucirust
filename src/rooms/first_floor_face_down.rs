use crate::{
    items::ItemId::*,
    rooms::Rooms::*,
    store::{actions, house::set_current_room, items::find_object, narration::simple_description},
};

super::generate_room!(
    "svgs/first_floor_face_down.svg",
    "On dirait un grand espace de jeu mal rangé.",
    [StairsFaceUp],
    [
        state,
        simple_description!("Shoes", "Un drôle de meuble à chaussures."),
        find_object!(
            state,
            "Closet",
            Strip3,
            "Un placard. Tiens, encore une de ces bandelettes.",
            "Un placard"
        ),
        simple_description!(
            "Box",
            r#"Un jeu d'échec et une note : "Une seule pièce vous manque et tout est dépeuplé.""#
        ),
        (
            "toRoomGui1",
            if state.house.is_door_to_room_gui1_open {
                || actions![set_current_room(RoomGui1)]
            } else {
                || actions![set_current_room(DoorGui1)]
            }
        ),
        (
            "toRoomMart1",
            if state.house.is_door_to_room_mart1_open {
                || actions![set_current_room(RoomMart1)]
            } else {
                || actions![set_current_room(DoorMart1)]
            }
        ),
        (
            "toRoomRom1",
            if state.house.is_room_rom1_open {
                || actions![set_current_room(RoomRom1)]
            } else {
                || actions![set_current_room(DoorRom1)]
            }
        ),
        (
            "toRoomTiph1",
            if state.house.is_door_to_room_tiph1_open {
                || actions![set_current_room(RoomTiph1)]
            } else {
                || actions![set_current_room(DoorTiph1)]
            }
        ),
    ],
);
