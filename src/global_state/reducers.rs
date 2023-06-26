use super::*;

use {std::rc::Rc, yew::prelude::*};

pub(crate) enum StateAction {
    Status(GameStatus),
}

impl Reducible for GlobalState {
    type Action = StateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next_state = (*self).clone();
        match action {
            StateAction::Status(status) => {
                next_state.game_status = status;
                next_state.into()
            }
        }
    }
}
