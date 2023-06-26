use crate::{GameStatus, GlobalState, StateAction};

use {
    web_sys::{Element, Node},
    yew::prelude::*,
};

#[function_component(Component)]
pub(crate) fn html() -> Html {
    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    let parsed =
        yew::Html::from_html_unchecked(yew::AttrValue::from(include_str!("hall_face_up.svg")));
    let onclick =
        { Callback::from(move |_| state.dispatch(StateAction::Status(GameStatus::Starting))) };
    use_effect(move || {
        let tree_of_hat = gloo::utils::document()
            .get_element_by_id("TreeOfHat")
            .expect("TreeOfHat not found in hall.svg");
        tree_of_hat
            .add_event_listener_with_callback(
                "click",
                &js_sys::Function::new_no_args(
                    r#"
                    var event = new CustomEvent("custard", {"bubbles": true});
                    temptest.dispatchEvent(event);
                "#,
                ),
            )
            .unwrap();
    });

    let div_node_ref = use_node_ref();

    use_effect_with_deps(
        {
            let div_node_ref = div_node_ref.clone();

            move |_| {
                let mut custard_listener = None;

                if let Some(element) = div_node_ref.cast::<web_sys::HtmlElement>() {
                    // Create your Callback as you normally would
                    let oncustard = onclick;

                    // Create a Closure from a Box<dyn Fn> - this has to be 'static
                    let listener =
                        gloo::events::EventListener::new(&element, "custard", move |e| {
                            oncustard.emit(e.clone())
                        });

                    custard_listener = Some(listener);
                }

                move || drop(custard_listener)
            }
        },
        div_node_ref.clone(),
    );

    html! {
        <div id="temptest" ref={div_node_ref} class="rooms_CurrentRoom">
            {parsed}
       // <div class="rooms_BlackVeil"></div>
        </div>
    }
}

fn on_click_tree_of_hat() -> yew::Callback<(), ()> {
    Callback::from(|_| panic!("test"))
}
