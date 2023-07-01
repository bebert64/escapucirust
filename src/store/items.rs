use super::GlobalStateAction;

use crate::items::{ItemFamily, ItemId, FAMILIES_BY_ID};

use {iter_tools::Itertools, std::collections::HashSet};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ItemsState {
    pub(crate) family_opened: Option<ItemFamily>,
    pub(crate) family_selected: Option<ItemFamily>,
    pub(crate) items_found: HashSet<ItemId>,
    pub(crate) items_in_inventory: HashSet<ItemId>,
}

#[derive(Clone, Debug)]
pub(crate) enum ItemsStateAction {
    OpenFamily(ItemFamily),
    CloseFamily,
    SelectFamily(ItemFamily),
    UnselectFamily,
    FindItem(ItemId),
    AddItemToInventory(ItemId),
    RemoveItemFromInventory(ItemId),
}

pub(super) fn reduce_items_state(action: ItemsStateAction, state: &mut ItemsState) {
    use ItemsStateAction::*;
    match action {
        OpenFamily(family) => state.family_opened = Some(family),
        CloseFamily => state.family_opened = None,
        SelectFamily(family) => state.family_selected = Some(family),
        UnselectFamily => state.family_selected = None,
        FindItem(item_id) => {
            state.items_found.insert(item_id);
        }
        AddItemToInventory(item_id) => {
            state.items_found.insert(item_id);
            state.items_in_inventory.insert(item_id);
        }
        RemoveItemFromInventory(item_id) => {
            state.items_in_inventory.remove(&item_id);
        }
    };
}

impl ItemsState {
    pub(crate) fn inventory(self: &Self) -> Vec<(ItemFamily, Vec<ItemId>)> {
        let mut inventory = self
            .items_in_inventory
            .iter()
            .map(|item_id| {
                (
                    *FAMILIES_BY_ID
                        .get(item_id)
                        .expect("All items should in map"),
                    *item_id,
                )
            })
            .into_group_map()
            .into_iter()
            .collect::<Vec<_>>();
        inventory.sort_by_key(|(family, _)| format!("{family:?}"));
        inventory
    }
}

pub(crate) fn open_family(family: ItemFamily) -> GlobalStateAction {
    GlobalStateAction::SetItemsState(ItemsStateAction::OpenFamily(family))
}

pub(crate) fn close_family() -> GlobalStateAction {
    GlobalStateAction::SetItemsState(ItemsStateAction::CloseFamily)
}

pub(crate) fn select_family(family: ItemFamily) -> GlobalStateAction {
    GlobalStateAction::SetItemsState(ItemsStateAction::SelectFamily(family))
}

pub(crate) fn unselect_family() -> GlobalStateAction {
    GlobalStateAction::SetItemsState(ItemsStateAction::UnselectFamily)
}

pub(crate) fn find_item(item_id: ItemId) -> GlobalStateAction {
    GlobalStateAction::SetItemsState(ItemsStateAction::FindItem(item_id))
}

pub(crate) fn add_item_to_inventory(item_id: ItemId) -> GlobalStateAction {
    GlobalStateAction::SetItemsState(ItemsStateAction::AddItemToInventory(item_id))
}

pub(crate) fn remove_item_from_iventory(item_id: ItemId) -> GlobalStateAction {
    GlobalStateAction::SetItemsState(ItemsStateAction::RemoveItemFromInventory(item_id))
}
