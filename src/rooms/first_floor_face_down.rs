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
    "svgs/first_floor_face_down.svg",
    "",
    [StairsFaceUp],
    [
        state,
        simple_description!(
            "Shoes", 
            "Un drôle de meuble à chaussures."),
        simple_description!(
            "Sofa", 
            "Il y a trop de bordel dessus pour pouvoir m'asseoir"),
        simple_description!(
            "Drawers", 
            "Des casses-têtes. Beaucoup de casses-tête. Non merci... je commence à en avoir ma claque."),
        find_object!(
            state,
            "Closet",
            Strip3,
            "Un, Deux, Trois, ... Il n'y a pas moins de 9 Monopoly différents.
            Tiens, encore une de ces bandelettes.",
            "Un, Deux, Trois, ... Il n'y a pas moins de 9 Monopoly différents"
        ),
        (
            "Box",
            if !state.house.is_door_gui1_open {
                || actions![set_current_room(ChessBoard)]
            } else {
                || {
                    actions![set_current_text(
                        "Un jeu d'échec, mais personne pour jouer avec moi..."
                    )]
                }
            }
        ),
        (
            "toRoomGui1",
            if state.house.is_door_gui1_open {
                || actions![set_current_room(RoomGui1)]
            } else {
                || actions![set_current_room(DoorGui1)]
            }
        ),
        (
            "toRoomMart1",
            if state.house.is_door_mart1_open {
                || actions![set_current_room(RoomMart1)]
            } else {
                || actions![set_current_room(DoorMart1)]
            }
        ),
        (
            "toRoomRom1",
            if state.house.is_door_rom1_open {
                || actions![set_current_room(RoomRom1)]
            } else {
                || actions![set_current_room(DoorRom1)]
            }
        ),
        (
            "toRoomTiph1",
            if state.house.is_door_tiph1_open {
                || actions![set_current_room(RoomTiph1)]
            } else {
                || actions![set_current_room(DoorTiph1)]
            }
        ),
    ],
    {
        let display_text_about_knight =
            !state.house.is_door_mart1_blocked && !state.items.items_found.contains(&Knight);
        let state = state.clone();
        use_effect_with_deps(
            move |display_text_about_knight: &bool| {
                state.dispatch(actions![set_current_text(if *display_text_about_knight {
                    "Au moment de mettre ce coup d'épaule, j'ai cru entendre un truc tomber. Probablement pas loin..."
                } else {
                    "On dirait un grand espace de jeu mal rangé."
                })])
            },
            display_text_about_knight,
        )
    }
);
