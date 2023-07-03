mod generation;

use generation::generate_items;

use {lazy_static::lazy_static, paste::paste, yew::prelude::*};

generate_items! {
    [ElectricalFuse, 1, 2, 3, 4, 5, 6];
    [Strip, 1, 2, 3, 4, 5, 6, 7];
    Saw;
    Board;
    Key;
}

#[derive(Properties, PartialEq)]
pub(crate) struct ItemProps {
    pub(crate) id: &'static str,
    pub(crate) family: ItemFamily,
    pub(crate) class: &'static str,
}

#[function_component(ItemComponent)]
pub(crate) fn html(props: &ItemProps) -> Html {
    let ItemProps { id, family, class } = props;
    use ItemFamily::*;
    let svg_str = match family {
        ElectricalFuse => include_str!("svgs/electrical_fuse.svg"),
        Saw => include_str!("svgs/saw.svg"),
        Strip => include_str!("svgs/strip.svg"),
        Board => include_str!("svgs/board.svg"),
        Key => include_str!("svgs/key.svg"),
    };
    let svg = yew::Html::from_html_unchecked(yew::AttrValue::from(svg_str));
    html! {
        <div id={*id} class={*class}>
            {svg}
        </ div>
    }
}
