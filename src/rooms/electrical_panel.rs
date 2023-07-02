use super::add_on_click_listener;

use crate::{
    items::{
        ItemFamily::*,
        ItemId::{self, *},
    },
    rooms::Rooms::*,
    store::{
        actions,
        house::{place_fuse, set_current_room, turn_light_on},
        items::{remove_item_from_iventory, unselect_family},
        narration::set_current_text,
    },
    GlobalState,
};

use {std::collections::HashSet, strum::IntoEnumIterator, strum_macros::EnumIter, yew::prelude::*};

#[derive(EnumIter, Debug, PartialEq, Eq, Hash, Clone)]
enum Fuse {
    Fuse1On,
    Fuse2On,
    Fuse3On,
    Fuse4On,
    Fuse5On,
    Fuse6On,
    Fuse1Off,
    Fuse2Off,
    Fuse3Off,
    Fuse4Off,
    Fuse5Off,
    Fuse6Off,
}

#[function_component(Room)]
pub(crate) fn html() -> Html {
    let room_ref = use_node_ref();
    let svg = yew::Html::from_html_unchecked(yew::AttrValue::from(include_str!(
        "svgs/electrical_panel.svg"
    )));
    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    let fuses_displayed = use_state(|| {
        let mut set = HashSet::from([Fuse::Fuse5Off, Fuse::Fuse6Off]);
        state
            .house
            .fuses_placed_on_electrical_panel
            .iter()
            .for_each(|&item_id| {
                set.insert(item_id.into_fuse_off());
            });
        set
    });

    // Set initial text
    let is_panel_ready = state.house.fuses_placed_on_electrical_panel.len() == 4;
    use_effect_with_deps(
        {
            let state = state.clone();
            move |_: &bool| {
                state.dispatch(actions![set_current_text({
                    if !is_panel_ready {
                        "OK, quelqu'un a enleve les fusibles, ils ne doivent pas etre bien loin"
                    } else {
                        "C'est bon, ca devrait marcher !"
                    }
                })]);
            }
        },
        is_panel_ready,
    );

    // Display and hide fuses based on local state
    use_effect_with_deps(
        {
            move |fuses_displayed: &UseStateHandle<HashSet<Fuse>>| {
                for fuse in Fuse::iter() {
                    let id = format!("{fuse:?}");
                    let fuse_elem = gloo::utils::document()
                        .get_element_by_id(&id)
                        .expect(&format!("{id} not found in svg"));
                    if !fuses_displayed.contains(&fuse) {
                        fuse_elem
                            .set_attribute("class", "hidden")
                            .expect("Problem setting {id}'s attribute");
                    } else {
                        fuse_elem
                            .set_attribute("class", "")
                            .expect("Problem setting {id}'s attribute");
                    }
                }
            }
        },
        fuses_displayed.clone(),
    );

    // Place fuses on panel
    let on_panel_click = {
        if state
            .items
            .family_selected
            .is_some_and(|family| family == ElectricalFuse)
        {
            let fuse_removed = state
                .items
                .inventory()
                .get_mut(&ElectricalFuse)
                .expect("If ElectricalFuse is selected, it should be in the inventory")
                .pop()
                .expect("No empty vec in inventory");
            let state = state.clone();
            Callback::from(move |_| {
                fuses_displayed.set({
                    let mut fuses = (*fuses_displayed).clone();
                    fuses.insert(fuse_removed.into_fuse_off());
                    fuses
                });
                state.dispatch(actions![
                    unselect_family(),
                    remove_item_from_iventory(fuse_removed),
                    place_fuse(fuse_removed)
                ]);
            })
        } else {
            Callback::default()
        }
    };

    // Toggle fuses
    // let interactions

    // Navigation
    {
        let state = state.clone();
        add_on_click_listener!(
            [("toHallFaceUp", move |_| state
                .dispatch(actions![set_current_room(HallFaceUp)]))],
            effect,
            room_ref
        );
        use_effect(effect);
    }

    html! {
        <div id="my_room" ref={room_ref} class="rooms_CurrentRoom" onclick={on_panel_click}>
            {svg}
        </div>
    }
}

impl ItemId {
    fn into_fuse_off(&self) -> Fuse {
        match self {
            ElectricalFuse1 => Fuse::Fuse1Off,
            ElectricalFuse2 => Fuse::Fuse2Off,
            ElectricalFuse3 => Fuse::Fuse3Off,
            ElectricalFuse4 => Fuse::Fuse4Off,
            _ => panic!("Should never been called with other variant"),
        }
    }
}
