use crate::{
    items::ItemId::*,
    rooms::Rooms::*,
    store::{actions, items::add_item_to_inventory, narration::set_current_text},
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
        ("Seats1", || actions![set_current_text(
            "On est bien là-dessus, mais ça ne va pas me faire sortir d'ici..."
        )]),
        (
            "Seats2",
            if state.items.items_found.contains(&Key) {
                || {
                    actions![set_current_text(
                        "On est bien là-dessus, mais ça ne va pas me faire sortir d'ici..."
                    )]
                }
            } else {
                || {
                    actions![set_current_text(
                        "On est bien là-dessus, mais ça ne va pas me faire sortir d'ici... Oh ! Une clef !"
                    ), add_item_to_inventory(Key)]
                }
            }
        ),
        ("Sofa1", || actions![set_current_text(
            "On est bien là-dessus, mais ça ne va pas me faire sortir d'ici..."
        )]),
        ("Sofa2", || actions![set_current_text(
            "On est bien là-dessus, mais ça ne va pas me faire sortir d'ici..."
        )]),
        ("Chimney", || actions![set_current_text(
            r#"un message gravé dans la pierre ... 'Seul un scout peut allumer cette cheminée'. Ok...
            "Quelques restes de prospectus à moitié brulés : mangez cinq fruits et légumes par jour... Ok."#
        )]),
        ("Window1", || actions![set_current_text(
            "Des grilles sont à la fenêtres. Surement pour empêcher d'entrer. Ou de sortir..."
        )]),
        ("Window2", || actions![set_current_text(
            "Des grilles sont à la fenêtres. Surement pour empêcher d'entrer. Ou de sortir..."
        )]),
        ("Window3", || actions![set_current_text(
            "Des grilles sont à la fenêtres. Surement pour empêcher d'entrer. Ou de sortir..."
        )]),
        ("Piano", || actions![set_current_text(
            "Il n'est pas accordé. Ceci dit, c'est la première fois que je joue du piano."
        )]),
        ("Bottles", || actions![set_current_text(
            r#"Il y a du très bon alcool là-dedans...
            Ah, il y a du très mauvais aussi !
            ...Du Patchaquoi ?"#
        )]),
        ("PhoneLamp", || actions![set_current_text(
            "C'est un téléhone-lampe ? Ou une lampe-téléphone ?"
        )]),
    ],
);
