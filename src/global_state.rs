use {std::rc::Rc, yew::prelude::*};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct GlobalState {
    pub(crate) game_status: GameStatus,
}

impl Default for GlobalState {
    fn default() -> GlobalState {
        GlobalState {
            game_status: GameStatus::Intro,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum GameStatus {
    Starting,
    Intro,
    Playing,
    Outro,
}

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
