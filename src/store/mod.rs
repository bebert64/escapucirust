pub(crate) mod game_status;
pub(crate) mod global_state;
pub(crate) mod house_state;
mod macros;
pub(crate) mod narration;

use {global_state::GlobalStateAction, macros::boolean_action};

pub(crate) use global_state::{GlobalState, GlobalStateActions};

#[derive(Clone, Debug)]
pub(crate) enum BooleanAction {
    True,
    False,
    Toggle,
}
