use crate::{GameStatus, GlobalState, StateAction};

use yew::{prelude::*, virtual_dom::VNode};

#[function_component(Component)]
pub(crate) fn html() -> Html {
    let page_index = use_state(|| 0);
    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    let onclick = {
        let index = page_index.clone();
        Callback::from(move |_| {
            if *index < PAGES.len() - 1 {
                index.set(*index + 1)
            } else {
                state.dispatch(StateAction::Status(GameStatus::Playing))
            }
        })
    };
    html! {
      <div class="Intro">
        <h1>{"Un soir d'été, dans un pays lointain"}</h1>
        {render_page(*page_index)}
        <br />
        <button className="startButton" {onclick}>{"Continuer"}</button>
      </div>
    }
}

struct Page<'a> {
    text: &'a str,
    img_url: &'a str,
}

const PAGES: [Page<'static>; 4] = [
    Page {
        text: "Damneud ! La voiture s'éteint sur un dernier toussotement. Il fait noir et il neige...",
        img_url: "static/intro_images/1.jpg",
    },
    Page {
        text: "Une seule maison aux alentours. Quelques fenetres semblent allumées. Une silhouete se détache.",
        img_url: "static/intro_images/2.jpg",
    },
    Page {
        text: "La porte est ouverte, j'entre ? Il doit bien y avoir quelqu'un à l'intérieur.",
        img_url: "static/intro_images/3.jpg",
    },
    Page {
        text: "BLAM ! Un courant d'air vient de claquer la porte... Pas de poignée de ce côté-ci. Et M... Damneud !",
        img_url: "static/intro_images/4.jpg",
    },
];

fn render_page(index: usize) -> VNode {
    let page = &PAGES[index];
    html! {
        <div class="Intro_img_and_text" >
            <img src={page.img_url} class="Intro_img"/>
            <br />
            <p >{page.text} </p>
        </div>
    }
}
