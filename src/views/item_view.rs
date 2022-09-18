use crate::models::domain::item::Item;

pub trait ItemDisplay {
    fn display_item_info(item: Item);
    fn edit_item_info(item: Item);
    fn get_item_info() -> Item;
}

pub struct ItemView {}
