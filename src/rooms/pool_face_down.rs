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
    "svgs/pool_face_down.svg",
    "EnToujours cette salle d'opération un peu glauque, on voit mal de ce côté.",
    [HallFaceUp, PoolFaceLeft],
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
        simple_description!("Boat1", "TrenteAnsPlusTard ? ok"),
        simple_description!(
            "Boat2",
            "C'est un fameux trois mâts. Il y a écrit 'Uyuni' ."
        ),
        find_object!(
            state,
            "Boat3",
            ElectricalFuse2,
            r#"Une maquette de bateau, un peu fragile.
                            Tiens, il y a quelque-chose cache la dessous..."#,
            "Oups, le mât est cassé... Cela reste beau quand meme. J'espère..."
        ),
        simple_description!(
            "UselessMirror",
            "Ca a du etre un mirroir à une époque. On n'y voit rien du tout."
        ),
        find_object!(
            state,
            "Box",
            Strip2,
            "Une boite pleine de trucs inutiles. Il y a quelque-chose dedans.",
            r#"Une boite pleine de trucs inutiles.
                        Il n'y a vraiment plus que des trucs inutiles dedans."#
        ),
        simple_description!(
            "Jukebox",
            "Dire Strait et ? C'est un mec célèbre, ça, Blondin ?"
        ),
        simple_description!(
            "PoolTable",
            "Un vieux billard qui a servi de table d'operation ? Ironique..."
        ),
    ],
);
