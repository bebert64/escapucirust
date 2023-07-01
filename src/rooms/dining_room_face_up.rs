use crate::{
    items::{ItemFamily, ItemId},
    rooms::Rooms::*,
    store::{
        actions,
        house::cut_table,
        items::{add_item_to_inventory, remove_item_from_iventory},
        narration::set_current_text,
    },
};

super::generate_room!(
    "svgs/dining_room_face_up.svg",
    "Oh ! La jolie Table",
    [LivingRoomFaceRight, KitchenFaceDown],
    [
        state,
        ("Door", || {
            actions![set_current_text(
                r#"Une porte vers l'extérieur. 
                Dommage que les devs aient eu la flemme de programmer un extérieur."#
            )]
        }),
        ("Drawing", || {
            actions![set_current_text(
                "Un tableau. Je n'arrive pas à me faire une opinion."
            )]
        }),
        ("Table", {
            if state.house.is_table_cut {
                || actions![set_current_text("Plus rien à faire.")]
            } else if state
                .items
                .family_selected
                .is_some_and(|family| family == ItemFamily::Saw)
            {
                || {
                    actions![
                        set_current_text("Cutting table."),
                        cut_table(),
                        remove_item_from_iventory(ItemId::Saw),
                        add_item_to_inventory(ItemId::Board)
                    ]
                }
            } else {
                || {
                    actions![set_current_text(
                        r#"Le bois de cette table pourrait servir de passerelle...
                        Mais elle est bien trop grande pour tenir dans mon sac !"#
                    )]
                }
            }
        })
    ],
);
