use crate::{
    rooms::Rooms::*,
    store::{actions, narration::set_current_text},
};

super::generate_room!(
    "svgs/living_room_face_right.svg",
    "Ce grand salon est vraiment vide",
    [DiningRoomFaceUp, PoolFaceDown, LivingRoomFaceUp],
    [
        state,
        ("Library", || actions![set_current_text(
            "Une vieille bibliothèque."
        )]),
        ("Chimney", || actions![set_current_text(
            r#"un message gravé dans la pierre ... 'Seul un scout peut allumer cette cheminée'. Ok...
            "Quelques restes de prospectus à moitié brulés : mangez cinq fruits et légumes par jour... Ok."#
        )]),
        ("Seats", || actions![set_current_text(
            "On est bien là-dessus, mais ça ne va pas me faire sortir d'ici..."
        )]),
        ("Sofa", || actions![set_current_text(
            "On est bien là-dessus, mais ça ne va pas me faire sortir d'ici..."
        )]),
        ("Window1", || actions![set_current_text(
            "Des grilles sont à la fenêtres. Surement pour empêcher d'entrer. Ou de sortir..."
        )]),
        ("Door", || actions![set_current_text(
            "Une porte vérouillée. Il va falloir trouver une autre issue."
        )]),
        ("Bottles", || actions![set_current_text(
            r#"Il y a du très bon alcool là-dedans...
            Ah, il y a du très mauvais aussi !
            ...Du Patchaquoi ?"#
        )]),
    ],
);
