macro_rules! generate_items {
    (
        $([$family: ident, $($id: expr),+]);* $(;)+
        $($unique_item: ident);* $(;)?
    ) => {
        paste! {
            #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
            pub(crate) enum ItemId {
                $($([<$family $id>]),*),*,
                $($unique_item),*,
            }

            #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
            pub(crate) enum ItemFamily {
                $($family),*,
                $($unique_item),*,
            }

            lazy_static! {
                pub(crate) static ref FAMILIES_BY_ID: ::std::collections::HashMap<ItemId, ItemFamily> = {
                    let mut map = ::std::collections::HashMap::new();
                    $($(
                        map.insert(
                            ItemId::[<$family $id>],
                            ItemFamily::$family,
                        );
                    )*)*
                    $(
                        map.insert(
                            ItemId::$unique_item,
                            ItemFamily::$unique_item,
                        );
                    )*
                    map
                };
            }
        }
    }
}

pub(crate) use generate_items;
