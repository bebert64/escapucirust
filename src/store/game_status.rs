use super::GlobalStateAction;

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

pub(crate) fn display_start_menu() -> GlobalStateAction {
    GlobalStateAction::SetGameStatus(GameStatus::Starting)
}

pub(crate) fn display_intro() -> GlobalStateAction {
    GlobalStateAction::SetGameStatus(GameStatus::Intro)
}

pub(crate) fn start_playing() -> GlobalStateAction {
    GlobalStateAction::SetGameStatus(GameStatus::Playing)
}

pub(crate) fn finish_game() -> GlobalStateAction {
    GlobalStateAction::SetGameStatus(GameStatus::Outro)
}
