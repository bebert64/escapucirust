use crate::{
    items::ItemId::*,
    rooms::Rooms::*,
    store::{
        actions, house::set_current_room, items::add_item_to_inventory, narration::set_current_text,
    },
};

super::generate_room!(
    "svgs/first_floor_face_down.svg",
    "On dirait un grand espace de jeu mal rangé.",
    [StairsFaceUp],
    [
        state,
        ("Shoes", || actions![set_current_text(
            "Un drôle de meuble à chaussures."
        )]),
        (
            "Closet",
            if !state.items.items_found.contains(&Strip3) {
                || {
                    actions![
                        set_current_text("Un placard. Tiens, encore une de ces bandelettes."),
                        add_item_to_inventory(Strip3)
                    ]
                }
            } else {
                || actions![set_current_text("Un placard")]
            }
        ),
        ("Box", || actions![set_current_text(
            r#"Un jeu d'échec et une note : "Une seule pièce vous manque et tout est dépeuplé.""#
        )]),
        (
            "toRoomGui1",
            if state.house.is_door_to_room_gui1_open {
                || actions![set_current_room(RoomGui1)]
            } else {
                || actions![set_current_room(DoorRoomGui1)]
            }
        ),
        (
            "toRoomMart1",
            if state.house.is_door_to_room_mart1_open {
                || actions![set_current_room(RoomMart1)]
            } else {
                || actions![set_current_room(DoorRoomMart1)]
            }
        ),
        (
            "toRoomRom1",
            if state.house.is_door_to_room_rom1_open {
                || actions![set_current_room(RoomRom1)]
            } else {
                || actions![set_current_room(DoorRoomRom1)]
            }
        ),
        (
            "toRoomTiph1",
            if state.house.is_door_to_room_tiph1_open {
                || actions![set_current_room(RoomTiph1)]
            } else {
                || actions![set_current_room(DoorRoomTiph1)]
            }
        ),
    ],
);
