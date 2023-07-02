use crate::{
    items::ItemId::*,
    rooms::Rooms::*,
    store::{
        actions, house::set_current_room, items::add_item_to_inventory, narration::set_current_text,
    },
};

super::generate_room!(
    "svgs/pool_face_left.svg",
    r#"On dirait une vieille salle d'operation,
    Il doit bien y avoir quelques trucs utiles a recuperer.."#,
    [HallFaceUp, PoolFaceDown],
    [
        state,
        (
            "toLivingRoomFaceUp",
            if state.house.is_light_on {
                || actions![set_current_room(LivingRoomFaceUp)]
            } else {
                || {
                    actions![set_current_text(
                        r#"Je n'irai pas plus loin sans lumière, c'est un coup 
                        à marcher sur une mine ou un vieux clou ou un LEGO"#
                    )]
                }
            }
        ),
        (
            "toStairsFaceUp",
            if state.house.is_light_on {
                || actions![set_current_room(StairsFaceUp)]
            } else {
                || {
                    actions![set_current_text(
                        r#"Je n'irai pas plus loin sans lumière, c'est un coup 
                        à marcher sur une mine ou un vieux clou ou un LEGO"#
                    )]
                }
            }
        ),
        ("BookShelf1", || actions![set_current_text(
            "De vieux livres de médecine tout écornés."
        )]),
        ("BookShelf2", || actions![set_current_text(
            "De vieux livres de psychologie, feuilletés par endroits."
        )]),
        ("BookShelf3", || {
            actions![
                set_current_text("La communication pour les nuls jamais ouvert, on dirait"),
                add_item_to_inventory(ElectricalFuse3)
            ]
        }),
        ("BookShelf4", || {
            actions![
                set_current_text(
                    r#"La cuisine vegan pour les nuls encore dans son emballage. 
                    Quelqu'un a dessiné une petite tête de mort dessus. Sympa."#
                ),
                add_item_to_inventory(ElectricalFuse4)
            ]
        }),
        ("BookShelf5", || actions![set_current_text(
            r#""Une vague pour Manu" ? Jamais entendu parler mais il y est 14 fois...
            Ca doit etre drolement bien, ou nul."#
        )]),
        (
            "Boat1",
            if !state.items.items_found.contains(&ElectricalFuse2) {
                || {
                    actions![
                        set_current_text(
                            r#"Une maquette de bateau, un peu fragile.
                            Tiens, il y a quelque-chose cache la dessous..."#
                        ),
                        add_item_to_inventory(ElectricalFuse2)
                    ]
                }
            } else {
                || {
                    actions![set_current_text(
                        "Oups, le mât est cassé... Cela reste beau quand meme. J'espère..."
                    )]
                }
            }
        ),
        ("Boat2", || actions![set_current_text(
            "C'est un fameux trois mâts. Il y a écrit 'Uyuni' ."
        )]),
        ("Frame1", || actions![set_current_text(
            "n cadre. On n'est pas obligés de faire des blagues à chaque fois, si ?"
        )]),
        ("Frame2", || actions![set_current_text(
            "Non mais le goût de gens qui vivaient ici..."
        )]),
        ("PoolTable", || actions![set_current_text(
            "n vieux billard qui a servi de table d'operation ? Ironique..."
        )]),
    ],
);
