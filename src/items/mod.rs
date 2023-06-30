mod generation;

use generation::generate_items;

use {lazy_static::lazy_static, paste::paste};

pub(crate) struct Item {
    id: ItemId,
    family: ItemFamily,
}

generate_items! {
    [Disjoncteur, 1, 2, 3, 4];
    [Bandelette, 5, 6];
    Saw;
}
