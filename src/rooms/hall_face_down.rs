use crate::{
    items::ItemId::*,
    rooms::Rooms::*,
    store::{actions, game_status::finish_game, items::find_object, narration::set_current_text},
};

super::generate_room!(
    "svgs/hall_face_down.svg",
    "On voit la sortie pourtant ! Il me faut cette poignée.",
    [HallFaceUp, PoolFaceLeft],
    [
        state,
        find_object!(
            state,
            "TreeOfHat",
            Strip1,
            r#"On dirait que ces chapeaux sont là depuis une éternité. Intrigant…
            J'en prendrais bien un mais on n'est pas le 2 mai, ça porte malheur.
            Ah tiens, une bandelette de papier ?"#,
            r#"On dirait que ces chapeaux sont là depuis une éternité. Intrigant…
            J'en prendrais bien un mais on n'est pas le 2 mai, ça porte malheur."#
        ),
        ("ExitDoor", {
            if !state.house.is_handle_on_exit_door {
                || {
                    actions![set_current_text(
                        r#"La porte s'est fermée toute seule, c'est pas rassurant, ça.
                        On dirait que la poignée a été retirée de ce côté. Impossible de sortir
                        Elle doit être quelque-part dans la maison..."#
                    )]
                }
            } else {
                || finish_game()
            }
        }),
    ],
);
