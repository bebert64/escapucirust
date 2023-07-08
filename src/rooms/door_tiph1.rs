use crate::{
    rooms::Rooms::*,
    store::{actions, house::set_current_room},
};

super::generate_room!(
    "svgs/electrical_panel.svg",
    r#"
    La porte de Tiph1 est bloqué et voilà un curieux cadenas à 7 lettres ressemblant 
    à s'y méprendre à un panneau électrique.
    "#,
    [],
    [("toHallFaceUp", {
        || actions![set_current_room(FirstFloorFaceDown)]
    }),],
);
