use crate::{
    items::ItemId::Handle,
    rooms::Rooms::*,
    store::{items::find_object, narration::simple_description},
};

super::generate_room!(
    "svgs/room_gui1.svg",
    "La chambre de Gui1",
    [FirstFloorFaceDown],
    [
        state,
        simple_description!(
            "LibraryGui",
            "Capitaine Tsubasa, Tom-Tom et Nana, et d'autres bouquins pour enfant"
        ),
        simple_description!(
            "DrawersGui",
            "Des affaires de foot et des t-shirts beaucoup trop grand pour un enfant"
        ),
        simple_description!(
            "Hatch",
            "C'est déjà bien assez effrayant comme ça... Pourquoi aller voir le grenier ?"
        ),
        find_object!(
            state,
            "Picture",
            Handle,
            r#"Ho, on voit la maison. Il y a beaucoup plus d'autres maisons que sur la photo de 
            quand je suis arrivé. Et une poignée est planquée derrière le cadre. Une poignée sans 
            porte. Il me semble avoir vu une porte sans poignée quelque part..."#,
            r#"Ho, on voit la maison. Il y a beaucoup plus d'autres maisons que sur la photo de 
            quand je suis arrivé."#,
        )
    ]
);
