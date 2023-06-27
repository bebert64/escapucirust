use super::{GlobalState, GlobalStateAction};

pub(super) fn reduce_narration(new_text: &'static str, state: &mut GlobalState) {
    state.current_text = new_text;
}

pub(crate) fn set_current_text(new_text: &'static str) -> GlobalStateAction {
    GlobalStateAction::SetCurrentText(new_text)
}
