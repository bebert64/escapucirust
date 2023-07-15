use crate::{
    items::ItemId::*,
    rooms::Rooms::*,
    store::{
        actions,
        house::set_current_room,
        items::add_item_to_inventory,
        narration::{set_current_text, simple_description},
    },
};

super::generate_room!(
    "svgs/room_tiph1.svg",
    "Une chambre de gonzesse, mais sans le rose",
    [FirstFloorFaceDown],
    [
        state,
        simple_description!(
            "LibraryTiph1",
            "Une autre page du journal. Pas utile pour me faire sortir d'ici mais intéressant."
        ),
        simple_description!(
            "Drawers",
            "'Comment réaliser un marathon en moins de 2h ?'. Tu peux toujours courir ma grande..."
        ),
        (
            "PieceOnFloor",
            if state.house.is_door_mart1_blocked {
                || {
                    actions![set_current_text(
                        "Rien. C'est propre, mais bon voilà, rien qui traine..."
                    )]
                }
            } else if !state.items.items_found.contains(&Knight) {
                || {
                    actions![
                        add_item_to_inventory(Knight),
                        set_current_text(
                            "Un cavalier blanc. Il a dû tomber vu comment j'ai trop de force"
                        )
                    ]
                }
            } else {
                || actions![set_current_text("Re plus rien...")]
            }
        ),
        ("Drawers", || actions![set_current_room(StripsPodium)])
    ]
);
