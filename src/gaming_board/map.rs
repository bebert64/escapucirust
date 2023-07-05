use crate::{rooms::Rooms::*, GlobalState};

use yew::prelude::*;

#[function_component(Map)]
pub(crate) fn html() -> Html {
    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    let first_floor_svg = include_str!("map_svgs/first_floor.svg");
    let ground_floor_svg = include_str!("map_svgs/ground_floor.svg");
    let (svg_str, current_room_map_id) = match state.house.current_room {
        DiningRoomFaceUp => (ground_floor_svg, "DiningRoom"),
        Drawers => (ground_floor_svg, "Kitchen"),
        ElectricalPanel => (ground_floor_svg, "Hall"),
        FirstFloorFaceDown => (first_floor_svg, "FirstFloor"),
        Frigogidaire => (ground_floor_svg, "Kitchen"),
        HallFaceDown => (ground_floor_svg, "Hall"),
        HallFaceUp => (ground_floor_svg, "Hall"),
        KitchenFaceDown => (ground_floor_svg, "Kitchen"),
        KitchenFaceLeft => (ground_floor_svg, "Kitchen"),
        LivingRoomFaceRight => (ground_floor_svg, "LivingRoom"),
        LivingRoomFaceUp => (ground_floor_svg, "LivingRoom"),
        PoolFaceDown => (ground_floor_svg, "Pool"),
        PoolFaceLeft => (ground_floor_svg, "Pool"),
        RoomGui1 => (first_floor_svg, "RoomGui1"),
        RoomMart1 => (first_floor_svg, "RoomMart1"),
        RoomTiph1 => (first_floor_svg, "RoomTiph1"),
        RoomRom1 => (first_floor_svg, "RoomRom1"),
        StairsFaceUp => (ground_floor_svg, "Stairs"),
        LockRoomGui1 => (first_floor_svg, "FirstFloor"),
        LockRoomMart1 => (first_floor_svg, "FirstFloor"),
        LockRoomRom1 => (first_floor_svg, "FirstFloor"),
        LockRoomTiph1 => (first_floor_svg, "FirstFloor"),
    };
    let svg = yew::Html::from_html_unchecked(yew::AttrValue::from(svg_str));
    use_effect(move || {
        let map = gloo::utils::document()
            .get_element_by_id("Map")
            .expect(&format!("{} not found in svg", "Map"));
        remove_class_from_descendants(map);
        let current_room = gloo::utils::document()
            .get_element_by_id(&current_room_map_id)
            .expect(&format!("{} not found in svg", &current_room_map_id));
        current_room
            .set_attribute("class", "board_MapCurrentRoom")
            .expect("Problem setting current_room's attribute");
    });

    html! {
        <div class="board_Map" id="Map">
            {svg}
        </ div>
    }
}

fn remove_class_from_descendants(elem: web_sys::Element) {
    let children = elem.children();
    for i in 0..children.length() {
        if let Some(child) = children.item(i) {
            child
                .set_attribute("class", "")
                .expect("Problem setting child's attribute");
            remove_class_from_descendants(child);
        };
    }
}
