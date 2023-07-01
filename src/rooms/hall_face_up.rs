use crate::{
    items::ItemId::*,
    rooms::Rooms::*,
    store::{
        actions, house::set_current_room, items::add_item_to_inventory, narration::set_current_text,
    },
};

super::generate_room!(
    "svgs/hall_face_up.svg",
    r#"
    Difficile d'avancer, on n'y voit rien du tout. 
    Cet endroit ne me dit rien qui vaille, je ne sais pas ce qui traine...
    Il vaut mieux ne pas trop m'éloigner avant d'avoir remis un peu de lumière.
    Cherchons.
    "#,
    [HallFaceDown, PoolFaceLeft, ElectricalPanel],
    [
        state,
        ("TreeOfHat", {
            if !state.items.items_found.contains(&Strip1) {
                || {
                    actions![
                        set_current_text(
                            r#"On dirait que ces chapeaux sont là depuis une éternité. Intrigant…
                            J'en prendrais bien un mais on n'est pas le 2 mai, ça porte malheur.
                            Ah tiens, une bandelette de papier ?"#
                        ),
                        add_item_to_inventory(Strip1)
                    ]
                }
            } else {
                || {
                    actions![set_current_text(
                        r#"On dirait que ces chapeaux sont là depuis une éternité. Intrigant…
                        J'en prendrais bien un mais on n'est pas le 2 mai, ça porte malheur."#
                    )]
                }
            }
        }),
        ("HallFrame", {
            if !state.items.items_found.contains(&ElectricalFuse1) {
                || {
                    actions![
                        set_current_text(
                            r#"Un collier de nouilles, mais en dessin.
                            Tiens, il y a quelque-chose de cache dessous..."#
                        ),
                        add_item_to_inventory(ElectricalFuse1)
                    ]
                }
            } else {
                || {
                    actions![set_current_text(
                        "Toujours le collier de nouilles, toujours en dessin."
                    )]
                }
            }
        }),
        ("toElectricalPanel", {
            if state.house.is_light_on {
                || {
                    actions![set_current_text(
                        "La lumière est allumee, plus besoin de ca..."
                    )]
                }
            } else {
                || actions![set_current_room(ElectricalPanel)]
            }
        })
    ],
);
