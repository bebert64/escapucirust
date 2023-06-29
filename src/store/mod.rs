mod boolean_action;
pub(crate) mod game_status;
pub(crate) mod global_state;
pub(crate) mod house_state;
pub(crate) mod narration;

use {boolean_action::boolean_action, global_state::GlobalStateAction};

pub(crate) use global_state::{GlobalState, GlobalStateActions};

#[derive(Clone, Debug)]
pub(crate) enum BooleanAction {
    True,
    False,
    Toggle,
}
