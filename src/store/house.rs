use super::{boolean_action, BooleanAction, GlobalStateAction};

use crate::rooms::Rooms;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct HouseState {
    pub(crate) current_room: Rooms,
    pub(crate) is_light_on: bool,
}

#[derive(Clone, Debug)]
pub(crate) enum HouseStateAction {
    SetLight(BooleanAction),
    SetCurrentRoom(Rooms),
}

pub(super) fn reduce_house_state(action: HouseStateAction, state: &mut HouseState) {
    use {BooleanAction::*, HouseStateAction::*};
    match action {
        SetLight(action) => boolean_action!(action, state.is_light_on),
        SetCurrentRoom(room) => state.current_room = room,
    };
}

pub(crate) fn toggle_light() -> GlobalStateAction {
    GlobalStateAction::SetHouseState(HouseStateAction::SetLight(BooleanAction::Toggle))
}

pub(crate) fn turn_on_light() -> GlobalStateAction {
    GlobalStateAction::SetHouseState(HouseStateAction::SetLight(BooleanAction::True))
}

pub(crate) fn turn_off_light() -> GlobalStateAction {
    GlobalStateAction::SetHouseState(HouseStateAction::SetLight(BooleanAction::False))
}

pub(crate) fn set_current_room(room: Rooms) -> GlobalStateAction {
    GlobalStateAction::SetHouseState(HouseStateAction::SetCurrentRoom(room))
}
