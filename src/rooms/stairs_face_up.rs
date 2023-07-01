use crate::rooms::Rooms::*;

super::generate_room!(
    "svgs/stairs_face_up.svg",
    "Enter stairs_face_up",
    [PoolFaceLeft, KitchenFaceLeft, FirstFloorFaceDown],
    [],
    {
        let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
        let effect = if state.items.items_found.contains(&crate::items::ItemId::Saw) {
            || {
                let board = gloo::utils::document()
                    .get_element_by_id("Board")
                    .expect("Board not found in svg");
                board.set_attribute("class", "hidden");
            }
        } else {
            || ()
        };
        use_effect(effect);
    }
);
