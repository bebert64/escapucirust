use super::GlobalStateAction;

use crate::{items::ItemId, rooms::Rooms};

use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct HouseState {
    pub(crate) current_room: Rooms,
    pub(crate) fuses_placed: HashSet<ItemId>,
    pub(crate) doudous_placed: HashSet<ItemId>,
    pub(crate) strips_placed: HashSet<ItemId>,
    pub(crate) is_light_on: bool,
    pub(crate) is_table_cut: bool,
    pub(crate) is_handle_on_exit_door: bool,
    pub(crate) is_board_on_hole: bool,
    pub(crate) is_door_gui1_open: bool,
    pub(crate) is_door_mart1_open: bool,
    pub(crate) is_door_rom1_open: bool,
    pub(crate) is_door_tiph1_open: bool,
    pub(crate) is_door_mart1_blocked: bool,
    pub(crate) is_knight_on_chess_board: bool,
    pub(crate) are_drawers_open: bool,
}

#[derive(Clone, Debug)]
pub(crate) enum HouseStateAction {
    TurnLightOn,
    SetCurrentRoom(Rooms),
    CutTable,
    PlaceFuse(ItemId),
    PlaceDoudou(ItemId),
    PlaceStrip(ItemId),
    PlaceBoardOnHole,
    OpenDrawers,
    OpenDoorRom1,
    OpenDoorTiph1,
    OpenDoorMart1,
    OpenDoorGui1,
    ForceDoorMart1,
    PlaceKnight,
    PlaceHandleOnExitDoor,
}

pub(super) fn reduce_house_state(action: HouseStateAction, state: &mut HouseState) {
    use HouseStateAction::*;
    match action {
        TurnLightOn => state.is_light_on = true,
        SetCurrentRoom(room) => state.current_room = room,
        CutTable => state.is_table_cut = true,
        PlaceFuse(item_id) => {
            state.fuses_placed.insert(item_id);
        }
        PlaceDoudou(item_id) => {
            state.doudous_placed.insert(item_id);
        }
        PlaceStrip(item_id) => {
            state.strips_placed.insert(item_id);
        }
        PlaceBoardOnHole => state.is_board_on_hole = true,
        OpenDrawers => state.are_drawers_open = true,
        OpenDoorRom1 => state.is_door_rom1_open = true,
        OpenDoorTiph1 => state.is_door_tiph1_open = true,
        OpenDoorMart1 => state.is_door_mart1_open = true,
        OpenDoorGui1 => state.is_door_gui1_open = true,
        ForceDoorMart1 => state.is_door_mart1_blocked = false,
        PlaceKnight => state.is_knight_on_chess_board = true,
        PlaceHandleOnExitDoor => state.is_handle_on_exit_door = true,
    };
}

pub(crate) fn turn_light_on() -> GlobalStateAction {
    GlobalStateAction::SetHouseState(HouseStateAction::TurnLightOn)
}

pub(crate) fn set_current_room(room: Rooms) -> GlobalStateAction {
    GlobalStateAction::SetHouseState(HouseStateAction::SetCurrentRoom(room))
}

pub(crate) fn cut_table() -> GlobalStateAction {
    GlobalStateAction::SetHouseState(HouseStateAction::CutTable)
}

pub(crate) fn place_fuse(item_id: ItemId) -> GlobalStateAction {
    GlobalStateAction::SetHouseState(HouseStateAction::PlaceFuse(item_id))
}

pub(crate) fn place_doudou(item_id: ItemId) -> GlobalStateAction {
    GlobalStateAction::SetHouseState(HouseStateAction::PlaceDoudou(item_id))
}

pub(crate) fn place_strip(item_id: ItemId) -> GlobalStateAction {
    GlobalStateAction::SetHouseState(HouseStateAction::PlaceStrip(item_id))
}

pub(crate) fn place_board_on_hole() -> GlobalStateAction {
    GlobalStateAction::SetHouseState(HouseStateAction::PlaceBoardOnHole)
}

pub(crate) fn open_drawers() -> GlobalStateAction {
    GlobalStateAction::SetHouseState(HouseStateAction::OpenDrawers)
}

pub(crate) fn open_door_rom1() -> GlobalStateAction {
    GlobalStateAction::SetHouseState(HouseStateAction::OpenDoorRom1)
}

pub(crate) fn open_door_tiph1() -> GlobalStateAction {
    GlobalStateAction::SetHouseState(HouseStateAction::OpenDoorTiph1)
}

pub(crate) fn open_door_mart1() -> GlobalStateAction {
    GlobalStateAction::SetHouseState(HouseStateAction::OpenDoorMart1)
}

pub(crate) fn open_door_gui1() -> GlobalStateAction {
    GlobalStateAction::SetHouseState(HouseStateAction::OpenDoorGui1)
}

pub(crate) fn force_door_mart1() -> GlobalStateAction {
    GlobalStateAction::SetHouseState(HouseStateAction::ForceDoorMart1)
}

pub(crate) fn place_knight() -> GlobalStateAction {
    GlobalStateAction::SetHouseState(HouseStateAction::PlaceKnight)
}

pub(crate) fn place_handle_on_exit_door() -> GlobalStateAction {
    GlobalStateAction::SetHouseState(HouseStateAction::PlaceHandleOnExitDoor)
}
