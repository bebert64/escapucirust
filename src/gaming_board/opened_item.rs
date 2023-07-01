use crate::{
    items::ItemComponent,
    store::{actions, items::close_family},
    GlobalState,
};

use yew::prelude::*;

#[function_component(OpenedItem)]
pub(crate) fn html() -> Html {
    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");

    if let Some(family) = state.items.family_opened {
        let on_background_click = {
            let state = state.clone();
            Callback::from(move |_| state.dispatch(actions![close_family()]))
        };
        let on_item_click = Callback::from(|event: MouseEvent| event.stop_propagation());
        html! {
            <div class="OpenedItemBackground" onclick={on_background_click}>
                <div class="OpenedItem" onclick={on_item_click}>
                    <ItemComponent id="2" family={family} class=""/>
                </div>
            </div>
        }
    } else {
        html! {
            <>
            </>
        }
    }
}
