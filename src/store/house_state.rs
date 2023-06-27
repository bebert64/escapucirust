use super::{boolean_action, BooleanAction, GlobalStateAction};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct HouseState {
    pub(crate) is_light_on: bool,
}

pub(crate) enum HouseStateAction {
    SetLight(BooleanAction),
}

pub(super) fn reduce_house_state(action: HouseStateAction, state: &mut HouseState) {
    use {BooleanAction::*, HouseStateAction::*};
    match action {
        SetLight(action) => boolean_action!(action, state.is_light_on),
    };
}

pub(crate) fn toggle_light() -> GlobalStateAction {
    GlobalStateAction::SetHouseState(HouseStateAction::SetLight(BooleanAction::Toggle))
}
