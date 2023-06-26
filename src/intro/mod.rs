use crate::{GameStatus, GlobalState, StateAction};

use yew::prelude::*;

#[function_component(Intro)]
pub(crate) fn html() -> Html {
    let index = use_state(|| 0);
    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    let onclick = {
        let index = index.clone();
        Callback::from(move |_| {
            if *index < 1 {
                index.set(*index + 1)
            } else {
                state.dispatch(StateAction::Status(GameStatus::Starting))
            }
        })
    };
    html! {
      <>
        <h1>{"Un soir d'été, dans un pays lointain"}</h1>
        {render_sequence(*index)}
        <br />
        <button className="startButton" {onclick}>{"Continuer"}</button>
      </>
    }
}

struct Page<'a> {
    text: &'a str,
    img_url: &'a str,
}

const PAGES: [Page<'static>; 2] = [
    Page {
        text: "1",
        img_url: "",
    },
    Page {
        text: "2",
        img_url: "",
    },
];

fn render_sequence(index: usize) -> String {
    return format!("index {}", PAGES[index].text);
}
