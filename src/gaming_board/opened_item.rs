use crate::{
    items::{ItemComponent, ItemFamily::*},
    store::{actions, items::close_family, narration::set_current_text},
    GlobalState,
};

use yew::prelude::*;

#[function_component(OpenedItem)]
pub(crate) fn html() -> Html {
    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");

    if let Some(family) = state.items.family_opened {
        state.dispatch(actions![set_current_text(match family {
            ElectricalFuse => "Un fusible, il a l'air en bon etat.",
            Saw => "Une scie",
            Strip => "Une bande, qui ressemble a s'y meprendre a un fusible.",
            Board => "Une planche de bois",
            Key => "Une clé",
            Doudou1 => "Un ours en peluche",
            Doudou2 => "Un ours en peluche",
            Doudou3 => "Un ours en peluche",
            Doudou4 => "Un ours en peluche",
            NoteDoorRom1 => "Cette note est très mystérieuse",
            NoteDoudous => "Cette note aussi est très mystérieuse. Que de mystères !",
            Knight => "Un cavalier blanc, tel le fameux",
            Handle => "Une poignée de porte en forme de fameux cavalier",
        })]);
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
