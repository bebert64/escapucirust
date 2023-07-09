use crate::{
    items::{ItemFamily, ItemId::*},
    rooms::Rooms::*,
    store::{
        actions,
        house::{open_drawers, set_current_room},
        items::remove_item_from_iventory,
        narration::{set_current_text, simple_description},
    },
};

super::generate_room!(
    "svgs/kitchen_face_down.svg",
    "Une vielle cuisine abandonnée... Tout a l'air un tout petit peu pourri..",
    [DiningRoomFaceUp, StairsFaceUp],
    [
        state,
        simple_description!("Shelf1", "On peut faire périmer du sel ??"),
        simple_description!(
            "Shelf2",
            r#"La Plancha pour les nuls. Avec post-it et annotations.
            Quelqu'un a barré "les nuls"."#
        ),
        simple_description!(
            "Drawer",
            "Des tupperwares sous des tupperwares, et entre d'autres tupperwares."
        ),
        simple_description!(
            "Door",
            "On dirait pas mais elle est fermée à clé. Clairement."
        ),
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
        ),
    ],
);
