use crate::{
    items::ItemComponent,
    store::{
        actions,
        items::{open_family, select_family, unselect_family},
    },
    GlobalState,
};

use yew::prelude::*;

const COLUMNS_QTY: usize = 2;
const ROWS_QTY: usize = 4;

#[function_component(Inventory)]
pub(crate) fn html() -> Html {
    let state = use_context::<UseReducerHandle<GlobalState>>().expect("Context not found");
    let inventory = state.items.inventory();
    let mut iventory_iter = inventory.into_iter();

    let mut rows = Vec::new();
    for _row in 0..ROWS_QTY {
        let mut cells = Vec::new();
        for _col in 0..COLUMNS_QTY {
            let mut td_class = "";
            let cell = if let Some((family, items)) = iventory_iter.next() {
                let is_family_selected = state
                    .items
                    .family_selected
                    .is_some_and(|family_selected| family == family_selected);
                if is_family_selected {
                    td_class = "itemSelected";
                } else {
                    td_class = "itemUnselected";
                }

                let qty = items.len();
                let qty_component = if qty > 1 {
                    html! {<p class="quantity">{qty}</p>}
                } else {
                    html! {}
                };

                let onclick = {
                    let state = state.clone();
                    Callback::from(move |_| {
                        if is_family_selected {
                            state.dispatch(actions!(unselect_family()));
                        } else {
                            state.dispatch(actions!(select_family(family)));
                        }
                    })
                };

                html! {
                    <div {onclick}>
                        <ItemComponent id="1" class="itemImage" family={family}/>
                        {qty_component}
                    </ div>
                }
            } else {
                html! {}
            };
            cells.push(html! {<td class={td_class}> {cell} </td>})
        }
        rows.push(html! {<tr>{cells}</tr>});
    }

    let on_open_click = {
        if let Some(family) = state.items.family_selected {
            let state = state.clone();
            Callback::from(move |_| state.dispatch(actions![open_family(family)]))
        } else {
            Callback::from(|_| ())
        }
    };

    html! {
        <div class="board_Inventory">
            <table>
                <tbody>{rows}</tbody>
            </table>
            <button class="OpenItemButton" onclick={on_open_click}>
                {"Ouvrir"}
            </button>
        </ div>
    }
}
