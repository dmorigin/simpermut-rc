

use slot::Slot;
use configuration::ReplacedItem;


#[derive(Default, Clone)]
pub struct Item
{
    pub id: String,
    pub slot: Slot,
    pub name: String,
    pub bonus_id: String,
    pub gem_id: String,
    pub relic_id: String,
    pub enchant_id: String,
}

impl Item {
    pub fn new() -> Item {
        Item {
            id: String::new(),
            slot: Slot::new(),
            name: String::new(),
            bonus_id: String::new(),
            gem_id: String::new(),
            relic_id: String::new(),
            enchant_id: String::new()
        }
    }

    pub fn from_replaced_item(other: &ReplacedItem) -> Item {
        Item {
            id: other.id.clone(),
            slot: Slot::from_str(&other.slot).unwrap(),
            name: other.name.clone(),
            bonus_id: other.bonus_id.clone(),
            gem_id: other.gem_id.clone(),
            relic_id: other.relic_id.clone(),
            enchant_id: other.enchant_id.clone()
        }
    }
}
