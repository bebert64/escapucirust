use super::{add_onclick_listener, create_listener, Rooms::*};

use crate::{
    items::ItemId::*,
    store::{
        actions,
        house::{open_door_tiph1, set_current_room},
        items::remove_item_from_iventory,
        narration::set_current_text,
    },
    GlobalState,
};

use {std::collections::HashMap, yew::prelude::*};

#[function_component(Room)]
pub(crate) fn html() -> Html {
    use Color::*;
    let room_ref = use_node_ref();
    let svg =
        yew::Html::from_html_unchecked(yew::AttrValue::from(include_str!("svgs/door_tiph1.svg")));
    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    let code_displayed = use_state(|| [Green, White, Green, Green]);

    // Set initial text
    use_effect_with_deps(
        {
            let state = state.clone();
            move |_| {
                state
                    .clone()
                    .dispatch(actions![set_current_text("Un cadenas à 4 couleurs",)])
            }
        },
        state.house.current_room.clone(),
    );

    // Display the current code based on local state
    {
        let state = state.clone();
        fn set_color(cell_index: usize, color: &Color) {
            let cell_id = format!("Cell{cell_index}");
            let cell_elem = gloo::utils::document()
                .get_element_by_id(&cell_id)
                .expect(&format!("{cell_id} not found in svg"));
            let cell_rgb = RGB_BY_COLOR.get(color).unwrap();
            cell_elem
                .set_attribute("style", &format!("fill:#{cell_rgb}"))
                .unwrap();
        }
        use_effect_with_deps(
            {
                move |code_displayed: &UseStateHandle<[Color; 4]>| {
                    if **code_displayed == [Green, White, Blue, Pink] {
                        state.dispatch(actions![
                            set_current_room(RoomTiph1),
                            open_door_tiph1(),
                            remove_item_from_iventory(NoteDoudous),
                        ]);
                    } else {
                        for i in 0..4 {
                            set_color(i, code_displayed.get(i).unwrap())
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
            for cell_index in 2..4 {
                let button_up_id = format!("ButtonUp{cell_index}");
                create_listener!(room_ref, listeners, button_up_id, {
                    let old_color = code_displayed.get(cell_index).unwrap();
                    let new_color = next_color(&old_color);
                    let code_displayed = code_displayed.clone();
                    move |_| {
                        code_displayed.set({
                            let mut new_code = (*code_displayed).clone();
                            let ref_to_modify = new_code.get_mut(cell_index).unwrap();
                            *ref_to_modify = new_color;
                            new_code
                        })
                    }
                });
                let button_down_id = format!("ButtonDown{cell_index}");
                create_listener!(room_ref, listeners, button_down_id, {
                    let old_color = code_displayed.get(cell_index).unwrap();
                    let new_color = previous_color(&old_color);
                    let code_displayed = code_displayed.clone();
                    move |_| {
                        code_displayed.set({
                            let mut new_code = (*code_displayed).clone();
                            let ref_to_modify = new_code.get_mut(cell_index).unwrap();
                            *ref_to_modify = new_color;
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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Color {
    Green,
    White,
    Pink,
    Blue,
}

lazy_static::lazy_static! {
    static ref RGB_BY_COLOR: HashMap<Color, &'static str> = {
        use Color::*;
        HashMap::from([
            (Green, "77ca4f"),
            (White, "FFFFFF"),
            (Pink, "dd8fcf"),
            (Blue, "1b68fe"),
        ])
    };
}

fn next_color(color: &Color) -> Color {
    use Color::*;
    match *color {
        Green => White,
        White => Pink,
        Pink => Blue,
        Blue => Green,
    }
}

fn previous_color(color: &Color) -> Color {
    use Color::*;
    match *color {
        Green => Blue,
        Blue => Pink,
        Pink => White,
        White => Green,
    }
}
