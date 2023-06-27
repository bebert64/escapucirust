use crate::rooms::Rooms::*;

super::generate_room!(
    "svgs/drawers.svg",
    "Enter drawers",
    [("zz", crate::store::house_state::toggle_light)],
    [KitchenFaceDown]
);
