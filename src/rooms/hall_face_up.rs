use crate::{
    rooms::Rooms::*,
    store::{
        house_state::{turn_off_light, turn_on_light},
        narration::set_current_text,
    },
};

super::generate_room!(
    "svgs/hall_face_up.svg",
    "Enter up & p",
    [("HallFrame", || set_current_text("Hall of frame"))],
    [("TreeOfHat", {
        let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
        if state.house_state.is_light_on {
            || vec![turn_off_light(), set_current_text("Lights off")]
        } else {
            || vec![turn_on_light(), set_current_text("Lights on")]
        }
    })],
    [HallFaceDown, ElectricalPanel, PoolFaceLeft]
);
