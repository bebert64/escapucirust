use super::{add_onclick_listener, create_listener};

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

use {std::collections::HashMap, yew::prelude::*};

#[function_component(Room)]
pub(crate) fn html() -> Html {
    let room_ref = use_node_ref();
    let svg = yew::Html::from_html_unchecked(yew::AttrValue::from(include_str!(
        "svgs/electrical_panel.svg"
    )));
    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    let fuses_displayed = use_state(|| {
        let mut set = HashMap::from([(ElectricalFuse5, false), (ElectricalFuse6, false)]);
        state.house.fuses_placed.iter().for_each(|&item_id| {
            set.insert(item_id, false);
        });
        set
    });

    // Set initial text
    let is_panel_ready = state.house.fuses_placed.len() == 4;
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

    // Display and hide fuses based on global and local state
    {
        let state = state.clone();
        fn set_class(fuse: &ItemId, is_on: bool, class: &str) {
            let fuse_element_id = fuse.into_fuse_element_id(is_on);
            let fuse_elem = gloo::utils::document()
                .get_element_by_id(fuse_element_id)
                .expect(&format!("{fuse_element_id} not found in svg"));
            fuse_elem
                .set_attribute("class", class)
                .expect("Problem setting {id}'s attribute");
        }
        use_effect_with_deps(
            {
                move |fuses_displayed: &UseStateHandle<HashMap<ItemId, bool>>| {
                    let mut are_all_fuses_on = true;
                    for (fuse, &is_on) in fuses_displayed.iter() {
                        are_all_fuses_on = are_all_fuses_on && is_on;
                        set_class(fuse, is_on, "show");
                        set_class(fuse, !is_on, "hidden");
                    }
                    if are_all_fuses_on {
                        state.dispatch(actions![
                            set_current_text("Et la lumière fut !"),
                            turn_light_on(),
                        ])
                    }
                }
            },
            fuses_displayed.clone(),
        );
    }

    // Place fuses on panel
    let on_panel_click = {
        if state
            .items
            .family_selected
            .is_some_and(|family| family == ElectricalFuse)
        {
            let fuse_to_place = *state
                .items
                .inventory()
                .get(&ElectricalFuse)
                .expect("If ElectricalFuse is selected, it should be in the inventory")
                .first()
                .expect("No empty vec in inventory");
            let state = state.clone();
            let fuses_displayed = fuses_displayed.clone();
            Callback::from(move |_| {
                fuses_displayed.set({
                    let mut fuses = (*fuses_displayed).clone();
                    fuses.insert(fuse_to_place, false);
                    fuses
                });
                state.dispatch(actions![
                    unselect_family(),
                    remove_item_from_iventory(fuse_to_place),
                    place_fuse(fuse_to_place)
                ]);
            })
        } else {
            Callback::default()
        }
    };

    // Toggle fuses
    {
        let room_ref = room_ref.clone();
        let fuses_displayed = fuses_displayed.clone();
        use_effect(move || {
            let mut listeners: Vec<Option<::gloo_events::EventListener>> = Vec::new();
            if is_panel_ready {
                for (fuse, is_on) in fuses_displayed.iter() {
                    let path_id = fuse.into_fuse_element_id(*is_on);
                    create_listener!(room_ref, listeners, path_id, {
                        let fuses_displayed = fuses_displayed.clone();
                        let fuses_toggled = FUSES_TOGGLED
                            .get(fuse)
                            .expect("All fuses should be in the hashmap");
                        move |_| {
                            fuses_displayed.set({
                                let mut fuses = (*fuses_displayed).clone();
                                for fuse in fuses_toggled {
                                    let is_on =
                                        fuses.get_mut(&fuse).expect("All fuses are in hashmap");
                                    *is_on = !*is_on;
                                }
                                fuses
                            })
                        }
                    });
                }
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
    fn into_fuse_element_id(&self, is_on: bool) -> &'static str {
        if is_on {
            match self {
                ElectricalFuse1 => "Fuse1On",
                ElectricalFuse2 => "Fuse2On",
                ElectricalFuse3 => "Fuse3On",
                ElectricalFuse4 => "Fuse4On",
                ElectricalFuse5 => "Fuse5On",
                ElectricalFuse6 => "Fuse6On",
                _ => panic!("Should never been called with other variant"),
            }
        } else {
            match self {
                ElectricalFuse1 => "Fuse1Off",
                ElectricalFuse2 => "Fuse2Off",
                ElectricalFuse3 => "Fuse3Off",
                ElectricalFuse4 => "Fuse4Off",
                ElectricalFuse5 => "Fuse5Off",
                ElectricalFuse6 => "Fuse6Off",
                _ => panic!("Should never been called with other variant"),
            }
        }
    }
}

lazy_static::lazy_static! {
    static ref FUSES_TOGGLED: HashMap<ItemId, Vec<ItemId>> =  HashMap::from([
        (ElectricalFuse1, vec![ElectricalFuse2]),
        (ElectricalFuse2, vec![ElectricalFuse1, ElectricalFuse3]),
        (ElectricalFuse3, vec![ElectricalFuse2, ElectricalFuse4]),
        (ElectricalFuse4, vec![ElectricalFuse3, ElectricalFuse5]),
        (ElectricalFuse5, vec![ElectricalFuse4, ElectricalFuse6]),
        (ElectricalFuse6, vec![ElectricalFuse5]),
    ]);
}
