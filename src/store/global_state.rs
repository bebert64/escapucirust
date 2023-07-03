use super::{
    game_status::{reduce_game_status, GameStatus},
    house::{reduce_house_state, HouseState, HouseStateAction},
    items::{reduce_items_state, ItemsState, ItemsStateAction},
    narration::reduce_narration,
};

use crate::rooms::Rooms;

use {
    std::{collections::HashSet, rc::Rc},
    yew::prelude::*,
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct GlobalState {
    pub(crate) game_status: GameStatus,
    pub(crate) house: HouseState,
    pub(crate) items: ItemsState,
    pub(crate) current_text: &'static str,
}

impl Default for GlobalState {
    fn default() -> GlobalState {
        GlobalState {
            game_status: GameStatus::Playing,
            house: HouseState {
                current_room: Rooms::LivingRoomFaceUp,
                is_light_on: true,
                is_table_cut: false,
                is_handle_on_exit_door: false,
                fuses_placed_on_electrical_panel: HashSet::new(),
            },
            items: ItemsState {
                family_opened: None,
                family_selected: None,
                items_found: HashSet::new(),
                items_in_inventory: HashSet::new(),
            },
            current_text: "Initial text",
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum GlobalStateAction {
    SetGameStatus(GameStatus),
    SetHouseState(HouseStateAction),
    SetCurrentText(&'static str),
    SetItemsState(ItemsStateAction),
}

#[derive(Clone, Debug)]
pub(crate) struct GlobalStateActions {
    pub(crate) actions: Vec<GlobalStateAction>,
}

impl Reducible for GlobalState {
    type Action = GlobalStateActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        use GlobalStateAction::*;
        let mut next_state = (*self).clone();
        action.actions.into_iter().for_each(|action| match action {
            SetGameStatus(status) => reduce_game_status(status, &mut next_state.game_status),
            SetHouseState(house_action) => reduce_house_state(house_action, &mut next_state.house),
            SetCurrentText(text) => reduce_narration(text, &mut next_state),
            SetItemsState(items_action) => reduce_items_state(items_action, &mut next_state.items),
        });
        next_state.into()
    }
}
