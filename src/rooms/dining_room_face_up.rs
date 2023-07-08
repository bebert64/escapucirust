use crate::{
    items::{
        ItemFamily,
        ItemId::{self, *},
    },
    rooms::Rooms::*,
    store::{
        actions,
        house::cut_table,
        items::{add_item_to_inventory, find_object, remove_item_from_iventory},
        narration::{set_current_text, simple_description},
    },
};

super::generate_room!(
    "svgs/dining_room_face_up.svg",
    "Oh ! La jolie Table",
    [LivingRoomFaceRight, KitchenFaceDown],
    [
        state,
        simple_description!(
            "Door",
            r#"Une porte vers l'extérieur. 
            Dommage que les devs aient eu la flemme de programmer un extérieur."#
        ),
        find_object!(
            state,
            "Drawing",
            Doudou2,
            "Un tableau. Je n'arrive pas à me faire une opinion. Un doudou était planqué derrière.",
            "Un tableau. Je n'arrive pas à me faire une opinion.",
        ),
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
                        add_item_to_inventory(Board)
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
    {
        use_effect(if state.house.is_table_cut {
            || {
                let table_cut = gloo::utils::document()
                    .get_element_by_id("TableCut")
                    .expect("TableCut not found in svg");
                table_cut
                    .set_attribute("class", "show")
                    .expect("Problem setting table_cut's attribute");
            }
        } else {
            || {
                let table_cut = gloo::utils::document()
                    .get_element_by_id("TableCut")
                    .expect("TableCut not found in svg");
                table_cut
                    .set_attribute("class", "hidden")
                    .expect("Problem setting table_cut's attribute");
            }
        });
    }
);
