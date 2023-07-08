use crate::{
    items::ItemId::*,
    rooms::Rooms::*,
    store::{items::find_object, narration::simple_description},
};

super::generate_room!(
    "svgs/frigogidaire.svg",
    "Est-ce un frigo ? Est-ce un frigidaire ?",
    [KitchenFaceLeft],
    [
        state,
        simple_description!("Shelf1", "Qui range sa vaisselle dans un frigogidaire ?"),
        simple_description!("Shelf2", "Ca ne se mange plus."),
        find_object!(
            state,
            "Shelf3",
            Strip3,
            "C'est périmé. Hmmm... Encore une bandelette.",
            "C'est périmé. Hmmm... "
        ),
    ],
);
