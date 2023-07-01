use crate::{
    items::ItemId::*,
    rooms::Rooms::*,
    store::{
        actions,
        house::{set_current_room, turn_off_light, turn_on_light},
        items::add_item_to_inventory,
        narration::set_current_text,
    },
};

super::generate_room!(
    "svgs/hall_face_up.svg",
    "Enter up & p",
    [
        ("TreeOfHat", {
            let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
            if state.house.is_light_on {
                || actions![turn_off_light(), set_current_text("Lights off")]
            } else {
                || actions![turn_on_light(), set_current_text("Lights on")]
            }
        }),
        ("HallFrame", || actions![
            set_current_text("Hall of frame"),
            add_item_to_inventory(Saw)
        ]),
        ("toPoolFaceLeft", {
            let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
            if state.house.is_light_on {
                || actions![set_current_room(PoolFaceLeft)]
            } else {
                || actions![set_current_text("Nope")]
            }
        })
    ],
    [HallFaceDown, ElectricalPanel]
);
