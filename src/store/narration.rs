use super::{GlobalState, GlobalStateAction};

pub(super) fn reduce_narration(new_text: &'static str, state: &mut GlobalState) {
    state.current_text = new_text;
}

pub(crate) fn set_current_text(new_text: &'static str) -> GlobalStateAction {
    GlobalStateAction::SetCurrentText(new_text)
}

macro_rules! simple_description {
    ($elem_id: expr, $description: expr) => {
        ($elem_id, || {
            crate::store::actions![crate::store::narration::set_current_text($description)]
        })
    };
}

pub(crate) use simple_description;
