pub(crate) mod dining_room_face_up;
pub(crate) mod drawers;
pub(crate) mod electrical_panel;
pub(crate) mod first_floor_face_down;
pub(crate) mod frigogidaire;
pub(crate) mod hall_face_down;
pub(crate) mod hall_face_up;
pub(crate) mod kitchen_face_down;
pub(crate) mod kitchen_face_left;
pub(crate) mod living_room_face_right;
pub(crate) mod living_room_face_up;
pub(crate) mod lock_room_gui1;
pub(crate) mod lock_room_mart1;
pub(crate) mod lock_room_rom1;
pub(crate) mod lock_room_tiph1;
pub(crate) mod pool_face_down;
pub(crate) mod pool_face_left;
pub(crate) mod room_gui1;
pub(crate) mod room_mart1;
pub(crate) mod room_rom1;
pub(crate) mod room_tiph1;
pub(crate) mod stairs_face_up;

mod generation;
mod onclick_listener;

use {
    generation::generate_room,
    onclick_listener::{
        add as add_on_click_listener, create_listener, create_listener_from_callback,
    },
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Rooms {
    DiningRoomFaceUp,
    Drawers,
    ElectricalPanel,
    FirstFloorFaceDown,
    Frigogidaire,
    HallFaceDown,
    HallFaceUp,
    KitchenFaceDown,
    KitchenFaceLeft,
    LivingRoomFaceRight,
    LivingRoomFaceUp,
    PoolFaceDown,
    PoolFaceLeft,
    RoomGui1,
    RoomMart1,
    RoomTiph1,
    RoomRom1,
    StairsFaceUp,
    LockRoomGui1,
    LockRoomMart1,
    LockRoomRom1,
    LockRoomTiph1,
}
