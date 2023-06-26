#[derive(Clone, Debug, PartialEq)]
pub(crate) struct GlobalState {
    pub(crate) game_status: GameStatus,
}

impl Default for GlobalState {
    fn default() -> GlobalState {
        GlobalState {
            game_status: GameStatus::Outro,
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
