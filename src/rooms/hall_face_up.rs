use crate::{
    rooms::Rooms::*,
    store::{
        game_status::display_start_menu, house_state::toggle_light, narration::set_current_text,
    },
};

super::generate_room!(
    "svgs/hall_face_up.svg",
    "Enter up & p",
    [
        ("HallFrame", display_start_menu),
        ("TreeOfHat", || set_current_text("Lights toggled")),
        ("TreeOfHat", toggle_light)
    ],
    [
        ("toHallFaceDown", HallFaceDown),
        ("toPoolFaceLeft", PoolFaceLeft)
    ]
);
