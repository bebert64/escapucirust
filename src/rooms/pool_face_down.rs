use crate::{
    items::ItemId::*,
    rooms::Rooms::*,
    store::{
        actions, house::set_current_room, items::add_item_to_inventory, narration::set_current_text,
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
        ("Boat1", || actions![set_current_text(
            "TrenteAnsPlusTard ? ok"
        )]),
        ("Boat2", || actions![set_current_text(
            "C'est un fameux trois mâts. Il y a écrit 'Uyuni' ."
        )]),
        (
            "Boat3",
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
        ("UselessMirror", || actions![set_current_text(
            "Ca a du etre un mirroir à une époque. On n'y voit rien du tout"
        )]),
        (
            "Box",
            if !state.items.items_found.contains(&Strip2) {
                || {
                    actions![
                        set_current_text(
                            r#"Une boite pleine de trucs inutiles
                            Il y a quelque-chose dedans"#
                        ),
                        add_item_to_inventory(Strip2)
                    ]
                }
            } else {
                || {
                    actions![set_current_text(
                        r#"Une boite pleine de trucs inutiles
                        Il n'y a vraiment plus que des trucs inutiles dedans"#
                    )]
                }
            }
        ),
        ("Jukebox", || actions![set_current_text(
            "Dire Strait et ? C'est un mec célèbre, ça, Blondin ?"
        )]),
        ("PoolTable", || actions![set_current_text(
            "Un vieux billard qui a servi de table d'operation ? Ironique..."
        )]),
    ],
);
