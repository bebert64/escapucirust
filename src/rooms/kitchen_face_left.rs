use crate::{
    items::ItemId::*,
    rooms::Rooms::*,
    store::{items::find_object, narration::simple_description},
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
        find_object!(
            state,
            "Drawer1",
            Doudou1,
            "Ah une peluche !",
            "Il n'y a plus rien dans ce tirroir.",
        ),
        simple_description!(
            "Drawer2",
            "Des conserves. De quoi faire deux canards entiers, en pièces détachées."
        ),
        simple_description!(
            "Scale",
            "Une balance à légumes, n'a visiblement jamais servi."
        ),
        simple_description!("PostalCards", r#""L'afrique est bonne hotesse?""#),
        simple_description!(
            "Drawing",
            "Marrant, ça par contre c'est pas mal !
            P'tetre le seul truc qui vaut quelque chose dans cette maison."
        ),
    ],
);
