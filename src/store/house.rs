use super::GlobalStateAction;

use crate::{items::ItemId, rooms::Rooms};

use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct HouseState {
    pub(crate) current_room: Rooms,
    pub(crate) fuses_placed_on_electrical_panel: HashSet<ItemId>,
    pub(crate) is_light_on: bool,
    pub(crate) is_table_cut: bool,
    pub(crate) is_handle_on_exit_door: bool,
}

#[derive(Clone, Debug)]
pub(crate) enum HouseStateAction {
    TurnLightOn,
    SetCurrentRoom(Rooms),
    CutTable,
    PlaceFuse(ItemId),
}

pub(super) fn reduce_house_state(action: HouseStateAction, state: &mut HouseState) {
    use HouseStateAction::*;
    match action {
        TurnLightOn => state.is_light_on = true,
        SetCurrentRoom(room) => state.current_room = room,
        CutTable => state.is_table_cut = true,
        PlaceFuse(item_id) => {
            state.fuses_placed_on_electrical_panel.insert(item_id);
        }
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
