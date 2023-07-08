use crate::{
    items::ItemId::*,
    rooms::Rooms::*,
    store::{actions, items::add_item_to_inventory, narration::set_current_text},
};

super::generate_room!(
    "svgs/kitchen_face_left.svg",
    "Cette cuisine est une catastrophe, un couloir, c'est pas vraiment pratique...",
    [
        KitchenFaceDown,
        Frigogidaire,
        DiningRoomFaceUp,
        StairsFaceUp
    ],
    [
        state,
        (
            "Drawer1",
            if !state.items.items_found.contains(&Doudou1) {
                || {
                    actions![
                        set_current_text("Ah une peluche !"),
                        add_item_to_inventory(Doudou1)
                    ]
                }
            } else {
                || actions![set_current_text("Il n'y a plus rien dans ce tirroir.")]
            }
        ),
        ("Drawer2", || actions![set_current_text(
            "Des conserves. De quoi faire deux canards entiers, en pièces détachées."
        )]),
        ("Scale", || actions![set_current_text(
            "Une balance à légumes, n'a visiblement jamais servi."
        )]),
        ("PostalCards", || actions![set_current_text(
            r#""L'afrique est bonne hotesse?""#
        )]),
        ("Drawing", || actions![set_current_text(
            "Marrant, ça par contre c'est pas mal !
            P'tetre le seul truc qui vaut quelque chose dans cette maison."
        )]),
    ],
);
