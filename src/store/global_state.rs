use super::{
    game_status::{reduce_game_status, GameStatus},
    house_state::{reduce_house_state, HouseState, HouseStateAction},
    narration::reduce_narration,
};

use crate::rooms::Rooms;

use {std::rc::Rc, yew::prelude::*};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct GlobalState {
    pub(crate) game_status: GameStatus,
    pub(crate) house_state: HouseState,
    pub(crate) current_text: &'static str,
}

impl Default for GlobalState {
    fn default() -> GlobalState {
        GlobalState {
            game_status: GameStatus::Playing,
            house_state: HouseState {
                current_room: Rooms::HallFaceUp,
                is_light_on: false,
            },
            current_text: "Wouhou",
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum GlobalStateAction {
    SetGameStatus(GameStatus),
    SetHouseState(HouseStateAction),
    SetCurrentText(&'static str),
}

impl Reducible for GlobalState {
    type Action = GlobalStateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        use GlobalStateAction::*;
        let mut next_state = (*self).clone();
        match action {
            SetGameStatus(status) => reduce_game_status(status, &mut next_state.game_status),
            SetHouseState(house_action) => {
                reduce_house_state(house_action, &mut next_state.house_state)
            }
            SetCurrentText(text) => reduce_narration(text, &mut next_state),
        };
        next_state.into()
    }
}
