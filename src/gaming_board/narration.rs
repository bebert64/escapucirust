use crate::{rooms::*, GlobalState};

use yew::prelude::*;

#[function_component(Component)]
pub(crate) fn html() -> Html {
    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");

    html! {
        <div class="board_Narration">
            <div class="Narration_textZone">
                <span class="Narration_text" id="currentText">
                    {state.current_text}
                </span>
            </div>
        </div>
    }
}
