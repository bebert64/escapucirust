use crate::{
    items::{ItemFamily, ItemId::*},
    rooms::Rooms::*,
    store::{
        actions,
        house::{open_drawers, set_current_room},
        items::remove_item_from_iventory,
        narration::set_current_text,
    },
};

super::generate_room!(
    "svgs/kitchen_face_down.svg",
    "Une vielle cuisine abandonnée... Tout a l'air un tout petit peu pourri..",
    [DiningRoomFaceUp, StairsFaceUp],
    [
        state,
        ("Shelf1", || actions![set_current_text(
            "On peut faire périmer du sel ??"
        )]),
        ("Shelf2", || actions![set_current_text(
            r#"La Plancha pour les nuls. Avec post-it et annotations.
            Quelqu'un a barré "les nuls"."#
        )]),
        ("Drawer", || actions![set_current_text(
            "Des tupperwares sous des tupperwares, et entre d'autres tupperwares."
        )]),
        ("Door", || actions![set_current_text(
            "On dirait pas mais elle est fermée à clé. Clairement."
        )]),
        (
            "toDrawers",
            if !state.house.are_drawers_open {
                if state
                    .items
                    .family_selected
                    .is_some_and(|family| family == ItemFamily::Key)
                {
                    || {
                        actions![
                            open_drawers(),
                            remove_item_from_iventory(Key),
                            set_current_room(Drawers)
                        ]
                    }
                } else {
                    || {
                        actions![set_current_text(
                            "C'est fermé à clé. Elle ne doit pas être bien loin."
                        )]
                    }
                }
            } else {
                || actions![set_current_room(Drawers)]
            }
        )
    ],
);
