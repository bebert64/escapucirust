macro_rules! generate_items {
    (
        $([$family: ident, $($id: expr),+]);* $(;)+
        $($unique_item: ident);* $(;)?
    ) => {
        paste! {
            #[derive(PartialEq, Eq, Hash)]
            enum ItemId {
                $($([<$family $id>]),*),*,
                $($unique_item),*,
            }

            enum ItemFamily {
                $($family),*,
                $($unique_item),*,
            }

            lazy_static! {
                static ref ITEMS: ::std::collections::HashMap<ItemId, Item> = {
                    let mut map = ::std::collections::HashMap::new();
                    $($(
                        map.insert(
                            ItemId::[<$family $id>],
                            Item {
                                family: ItemFamily::$family,
                                id: ItemId::[<$family $id>],
                            }
                        );
                    )*)*
                    $(
                        map.insert(
                            ItemId::$unique_item,
                            Item {
                                family: ItemFamily::$unique_item,
                                id: ItemId::$unique_item,
                            }
                        );
                    )*
                    map
                };
            }
        }
    }
}

pub(crate) use generate_items;
