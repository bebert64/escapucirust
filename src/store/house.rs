use super::GlobalStateAction;

use crate::rooms::Rooms;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct HouseState {
    pub(crate) current_room: Rooms,
    pub(crate) is_light_on: bool,
    pub(crate) is_table_cut: bool,
    pub(crate) is_handle_on_exit_door: bool,
}

#[derive(Clone, Debug)]
pub(crate) enum HouseStateAction {
    TurnLightOn,
    SetCurrentRoom(Rooms),
    CutTable,
}

pub(super) fn reduce_house_state(action: HouseStateAction, state: &mut HouseState) {
    use HouseStateAction::*;
    match action {
        TurnLightOn => state.is_light_on = true,
        SetCurrentRoom(room) => state.current_room = room,
        CutTable => state.is_table_cut = true,
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
