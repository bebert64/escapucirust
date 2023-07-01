pub(crate) mod game_status;
pub(crate) mod global_state;
pub(crate) mod house;
pub(crate) mod items;
pub(crate) mod narration;

use global_state::GlobalStateAction;

pub(crate) use global_state::{GlobalState, GlobalStateActions};

macro_rules! actions {
    ($($action: expr),+) => {
        crate::store::global_state::GlobalStateActions {
            actions: vec![$($action),+]
        }
    }
}

pub(crate) use actions;
