use crate::{
    items::ItemId::*,
    rooms::Rooms::*,
    store::{items::find_object, narration::simple_description},
};

super::generate_room!(
    "svgs/living_room_face_right.svg",
    "Ce grand salon est vraiment vide",
    [DiningRoomFaceUp, PoolFaceDown, LivingRoomFaceUp],
    [
        state,
        simple_description!("Library", "Une vieille bibliothèque."),
        find_object!(
            state,
            "Chimney",
            Doudou3,
            r#"Un doudou est caché parmi les cendres. Il est en parfait état, c'est incroyable."#,
            r#"un message gravé dans la pierre ... 'Seul un scout peut allumer cette cheminée'. Ok...
            "Quelques restes de prospectus à moitié brulés : mangez cinq fruits et légumes par jour... Ok."#
        ),
        simple_description!(
            "Seats",
            "On est bien là-dessus, mais ça ne va pas me faire sortir d'ici..."
        ),
        simple_description!(
            "Sofa",
            "On est bien là-dessus, mais ça ne va pas me faire sortir d'ici..."
        ),
        simple_description!(
            "Window1",
            "Des grilles sont à la fenêtres. Surement pour empêcher d'entrer. Ou de sortir..."
        ),
        simple_description!(
            "Door",
            "Une porte vérouillée. Il va falloir trouver une autre issue."
        ),
        simple_description!(
            "Bottles",
            r#"Il y a du très bon alcool là-dedans...
            Ah, il y a du très mauvais aussi !
            ...Du Patchaquoi ?"#
        ),
    ],
);
