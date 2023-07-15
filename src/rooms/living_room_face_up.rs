use crate::{
    items::ItemId::*,
    rooms::Rooms::*,
    store::{items::find_object, narration::simple_description},
};

super::generate_room!(
    "svgs/living_room_face_up.svg",
    r#"Le salon
    avec des beaux canapes
    et une cheminee
    poil au nez"#,
    [DiningRoomFaceUp, PoolFaceDown, LivingRoomFaceRight],
    [
        state,
        simple_description!(
            "Seats1",
            "On est bien là-dessus, mais ça ne va pas me faire sortir d'ici..."
        ),
        find_object!(
            state,
            "Seats2",
            Key,
            "On est bien là-dessus, mais ça ne va pas me faire sortir d'ici...",
            "On est bien là-dessus, mais ça ne va pas me faire sortir d'ici... Oh ! Une clef !"
        ),
        simple_description!(
            "Sofa1",
            "On est bien là-dessus, mais ça ne va pas me faire sortir d'ici..."
        ),
        simple_description!(
            "Sofa2",
            "On est bien là-dessus, mais ça ne va pas me faire sortir d'ici..."
        ),
        find_object!(
            state,
            "Chimney",
            Doudou3,
            r#"Un doudou est caché parmi les cendres. Il est en parfait état, c'est incroyable."#,
            r#"un message gravé dans la pierre ... 'Seul un scout peut allumer cette cheminée'. Ok...
            "Quelques restes de prospectus à moitié brulés : mangez cinq fruits et légumes par jour... Ok."#
        ),
        simple_description!(
            "Window1",
            "Des grilles sont à la fenêtres. Surement pour empêcher d'entrer. Ou de sortir..."
        ),
        simple_description!(
            "Window2",
            "Des grilles sont à la fenêtres. Surement pour empêcher d'entrer. Ou de sortir..."
        ),
        simple_description!(
            "Window3",
            "Des grilles sont à la fenêtres. Surement pour empêcher d'entrer. Ou de sortir..."
        ),
        find_object!(
            state,
            "Piano",
            Doudou4,
            "Il n'est pas accordé. Ceci dit, c'est la première fois que je joue du piano. Ou alors, c'est à cause du doudou planqué dedans ?",
            "Ha ben non, ça devait être moi...",
        ),
        simple_description!(
            "Bottles",
            r#"Il y a du très bon alcool là-dedans...
            Ah, il y a du très mauvais aussi !
            ...Du Patchaquoi ?"#
        ),
        simple_description!(
            "PhoneLamp",
            "C'est un téléphone-lampe ? Ou une lampe-téléphone ?"
        ),
    ],
);
