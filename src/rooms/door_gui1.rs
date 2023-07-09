use super::{add_onclick_listener, create_listener, Rooms::*};

use crate::{
    store::{
        actions,
        house::{open_door_gui1, set_current_room},
        narration::set_current_text,
    },
    GlobalState,
};

use yew::prelude::*;

#[function_component(Room)]
pub(crate) fn html() -> Html {
    let room_ref = use_node_ref();
    let svg =
        yew::Html::from_html_unchecked(yew::AttrValue::from(include_str!("svgs/door_gui1.svg")));
    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    let code_displayed = use_state(|| [0, 0, 0, 1, 1]);
    let current_code = (*code_displayed)
        .iter()
        .enumerate()
        .map(|(cell_index, letter_index)| {
            *CELL_LETTERS
                .get(cell_index)
                .unwrap()
                .get(*letter_index as usize)
                .unwrap()
        })
        .collect::<Vec<_>>()
        .join("");

    // Set initial text
    use_effect_with_deps(
        {
            let state = state.clone();
            move |_| {
                state
                    .clone()
                    .dispatch(actions![set_current_text("Un cadenas à 7 lettres",)])
            }
        },
        state.house.current_room.clone(),
    );

    // Display the current code based on local state
    {
        fn set_text(cell_index: usize, letter_index: &i32) {
            let cell_id = format!("Text{cell_index}");
            let cell_elem = gloo::utils::document()
                .get_element_by_id(&cell_id)
                .expect(&format!("{cell_id} not found in svg"));
            let tspan = cell_elem.first_child().unwrap();
            tspan.set_text_content(Some(
                &CELL_LETTERS
                    .get(cell_index)
                    .unwrap()
                    .get(*letter_index as usize)
                    .unwrap()
                    .to_string(),
            ));
        }
        use_effect_with_deps(
            {
                let state = state.clone();
                move |code_displayed: &UseStateHandle<[i32; 5]>| {
                    if current_code == "VERT " {
                        state.dispatch(actions![set_current_room(RoomGui1), open_door_gui1(),]);
                    } else {
                        for i in 0..5 {
                            set_text(i, code_displayed.get(i).unwrap())
                        }
                    }
                }
            },
            code_displayed.clone(),
        );
    }

    // Connect_buttons
    {
        let room_ref = room_ref.clone();
        let code_displayed = code_displayed.clone();
        use_effect(move || {
            let mut listeners: Vec<Option<::gloo_events::EventListener>> = Vec::new();
            for cell_index in 3..5 {
                let button_up_id = format!("ButtonUp{cell_index}");
                create_listener!(room_ref, listeners, button_up_id, {
                    let cell_letters = CELL_LETTERS.get(cell_index).unwrap();
                    let next_cell_code_displayed = (code_displayed.get(cell_index).unwrap() + 1)
                        .rem_euclid(cell_letters.len() as i32);
                    let code_displayed = code_displayed.clone();
                    move |_| {
                        code_displayed.set({
                            let mut new_code = (*code_displayed).clone();
                            let ref_to_modify = new_code.get_mut(cell_index).unwrap();
                            *ref_to_modify = next_cell_code_displayed;
                            new_code
                        })
                    }
                });
                let button_down_id = format!("ButtonDown{cell_index}");
                create_listener!(room_ref, listeners, button_down_id, {
                    let cell_letters = CELL_LETTERS.get(cell_index).unwrap();
                    let next_cell_code_displayed = (code_displayed.get(cell_index).unwrap() - 1)
                        .rem_euclid(cell_letters.len() as i32);
                    let code_displayed = code_displayed.clone();
                    move |_| {
                        code_displayed.set({
                            let mut new_code = (*code_displayed).clone();
                            let ref_to_modify = new_code.get_mut(cell_index).unwrap();
                            *ref_to_modify = next_cell_code_displayed;
                            new_code
                        })
                    }
                });
            }
            move || {
                drop(listeners);
            }
        });
    }

    // Navigation
    {
        let state = state.clone();
        add_onclick_listener!(
            [("toFirstFloorFaceDown", move |_| state
                .dispatch(actions![set_current_room(FirstFloorFaceDown)]))],
            effect,
            room_ref
        );
        use_effect(effect);
    }

    html! {
        <div id="my_room" ref={room_ref} class="rooms_CurrentRoom" >
            {svg}
        </div>
    }
}

lazy_static::lazy_static! {
    static ref CELL_LETTERS: Vec<Vec<&'static str>> =  vec![
        vec!["V", "S", "V"],
        vec!["E", "S", "V"],
        vec!["R", "S", "V"],
        vec!["T", "S", "V"],
        vec![" ", "S", "V"],
    ];
}
