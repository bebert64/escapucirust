use crate::{
    items::ItemId::*,
    rooms::Rooms::*,
    store::{
        actions, game_status::finish_game, items::add_item_to_inventory,
        narration::set_current_text,
    },
};

super::generate_room!(
    "svgs/frigogidaire.svg",
    "Est-ce un frigo ? Est-ce un frigidaire ?",
    [HallFaceUp, PoolFaceLeft],
    [
        state,
        ("Shelf1", {
            || {
                actions![set_current_text(
                    "Qui range sa vaisselle dans un frigogidaire ?"
                )]
            }
        }),
        ("Shelf2", {
            || actions![set_current_text("Ca ne se mange plus.")]
        }),
        ("Shelf3", {
            if !state.items.items_found.contains(&Strip3) {
                || {
                    actions![
                        set_current_text("C'est périmé. Hmmm... Encore une bandelette."),
                        add_item_to_inventory(Strip3)
                    ]
                }
            } else {
                || actions![set_current_text("C'est périmé. Hmmm... ")]
            }
        }),
    ],
);
