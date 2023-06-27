use super::{global_state::GlobalStateAction::*, GlobalStateActions};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum GameStatus {
    Starting,
    Intro,
    Playing,
    Outro,
}

pub(super) fn reduce_game_status(action: GameStatus, state: &mut GameStatus) {
    *state = action;
}

pub(crate) fn display_start_menu() -> GlobalStateActions {
    GlobalStateActions {
        actions: vec![SetGameStatus(GameStatus::Starting)],
    }
}

pub(crate) fn display_intro() -> GlobalStateActions {
    GlobalStateActions {
        actions: vec![SetGameStatus(GameStatus::Intro)],
    }
}

pub(crate) fn start_playing() -> GlobalStateActions {
    GlobalStateActions {
        actions: vec![SetGameStatus(GameStatus::Playing)],
    }
}

pub(crate) fn finish_game() -> GlobalStateActions {
    GlobalStateActions {
        actions: vec![SetGameStatus(GameStatus::Outro)],
    }
}
