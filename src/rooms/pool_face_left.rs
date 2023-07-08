use crate::{
    items::ItemId::*,
    rooms::Rooms::*,
    store::{
        actions,
        house::set_current_room,
        items::find_object,
        narration::{set_current_text, simple_description},
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
        simple_description!("BookShelf1", "De vieux livres de médecine tout écornés."),
        simple_description!(
            "BookShelf2",
            "De vieux livres de psychologie, feuilletés par endroits."
        ),
        find_object!(
            state,
            "BookShelf3",
            ElectricalFuse3,
            r#"La communication pour les nuls jamais ouvert, on dirait.
                Mais que vois-je ? Un fusible caché dedans ?"#,
            "La communication pour les nuls jamais ouvert, on dirait"
        ),
        find_object!(
            state,
            "BookShelf4",
            ElectricalFuse4,
            r#"La cuisine vegan pour les nuls encore dans son emballage. 
                Quelqu'un a dessiné une petite tête de mort dessus. Sympa.
                Mais que vois-je ? Un fusible caché dedans ?"#,
            r#"La cuisine vegan pour les nuls encore dans son emballage. 
                Quelqu'un a dessiné une petite tête de mort dessus. Sympa."#
        ),
        simple_description!(
            "BookShelf5",
            r#""Une vague pour Manu" ? Jamais entendu parler mais il y est 14 fois...
            Ca doit etre drolement bien, ou nul."#
        ),
        find_object!(
            state,
            "Boat1",
            ElectricalFuse2,
            r#"Une maquette de bateau, un peu fragile.
                Tiens, il y a quelque-chose cache la dessous..."#,
            "Oups, le mât est cassé... Cela reste beau quand meme. J'espère..."
        ),
        simple_description!(
            "Boat2",
            "C'est un fameux trois mâts. Il y a écrit 'Uyuni' ."
        ),
        simple_description!(
            "Frame1",
            "Un cadre. On n'est pas obligés de faire des blagues à chaque fois, si ?"
        ),
        find_object!(
            state,
            "Frame2",
            NoteDoorRom1,
            "Non mais le goût de gens qui vivaient ici... Oh, une note !",
            "Non mais le goût de gens qui vivaient ici...",
        ),
        simple_description!(
            "PoolTable",
            "Un vieux billard qui a servi de table d'operation ? Ironique..."
        ),
    ],
);
