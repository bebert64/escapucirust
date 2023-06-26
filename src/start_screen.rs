use crate::{GameStatus, GlobalState, StateAction};

use yew::{prelude::*, virtual_dom::VNode};

#[function_component(Component)]
pub(crate) fn html() -> Html {
    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    let open_tab = use_state(|| Tab::Team);
    let onclick_start =
        Callback::from(move |_| state.dispatch(StateAction::Status(GameStatus::Intro)));
    let onclick_tab = |new_tab: Tab| {
        let tab = open_tab.clone();
        Callback::from(move |_| tab.set(new_tab.clone()))
    };

    html! {
        <>
        <header class="start_header">
        <h1>{"Welcome to Escapucina !"}</h1>
        <img src="static/maison_red_car.jpg" class="start_coverPageImage" alt="" />
        </header>

        <button class="start_startButton" onclick={onclick_start}>{"Démarrer l'aventure..."}</button>

        <div class="start_tabs">
            <div class="start_buttons">
                <button class="start_tablinks" id="TeamButton" onclick={onclick_tab(Tab::Team)}>{"L'équipe"}</button>
                <button class="start_tablinks" id="ThanksButton" onclick={onclick_tab(Tab::Thanks)}>{"Remerciements"}</button>
                <button class="start_tablinks" id="SorryButton" onclick={onclick_tab(Tab::Sorry)}>{"Excuses"}</button>
            </div>
            {tab(&(*open_tab))}
        </div>

        </>
    }
}

#[derive(Clone)]
enum Tab {
    Team,
    Thanks,
    Sorry,
}

fn tab(open_tab: &Tab) -> VNode {
    match open_tab {
        &Tab::Team => {
            html! {
                <div class="start_tabcontent">
                    <h3>{"Par Ordre alphabétique d'utilité"}	</h3>
                    <li>{"Rom-1		Technique"}</li>
                    <li>{"Tiph-1		Consulting"}</li>
                    <li>{"Mart-1		Design"}</li>
                    <li>{"Gui-1		Enigmes"}</li>
                    <br />
                </div>
            }
        }
        &Tab::Thanks => {
            html! {
                <div class="start_tabcontent">
                    <p>
                    {"Est-ce vraiment notre genre ?"} <br />
                    {"Non sérieusement, ce jeu est l'aboutissement d'une éducation
                    ouverte et irréverencieuse, dans une maison qui l'est tout
                    autant."}<br />
                    {"Alors, probablement, merci à tous ceux qui y ont particité de
                        près ou de loin."}
                    </p>
                </div>
            }
        }
        &Tab::Sorry => {
            html! {
                <div class="start_tabcontent">
                <p>
                {"Pas notre genre non plus."}<br />
                {"Mais bon, si dans notre désir de remettre toute cette histoire
                en lumière, il est arrivé qu'on froisse un nerf, ou un égo,
                nous présentons par avance des excuses moyennement sincères."} <br />
                {"Un peu d'humour, merde."}
                </p>
                </div>
            }
        }
    }
}
