pub(crate) mod game_status;
pub(crate) mod global_state;
pub(crate) mod house_state;
mod macros;

use {global_state::GlobalStateAction, macros::boolean_action};

pub(crate) use global_state::GlobalState;

pub(crate) enum BooleanAction {
    True,
    False,
    Toggle,
}
