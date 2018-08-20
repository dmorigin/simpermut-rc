

use slot::Slot;
use configuration::ReplacedItem;


#[derive(Default, Clone)]
pub struct Item
{
    pub id: u32,
    pub slot: Slot,
    pub name: String,
    pub bonus_id: String,
    pub gem_id: String,
    pub relic_id: String,
    pub enchant_id: u32,
    pub azerite_power: String,
    pub azerite_level: u32
}

impl Item {
    pub fn new() -> Item {
        Item {
            id: 0,
            slot: Slot::new(),
            name: String::new(),
            bonus_id: String::new(),
            gem_id: String::new(),
            relic_id: String::new(),
            enchant_id: 0,
            azerite_power: String::new(),
            azerite_level: 0
        }
    }

    pub fn from_replaced_item(other: &ReplacedItem) -> Item {
        Item {
            id: other.id,
            slot: Slot::from_str(&other.slot).unwrap(),
            name: other.name.clone(),
            bonus_id: other.bonus_id.clone(),
            gem_id: other.gem_id.clone(),
            relic_id: other.relic_id.clone(),
            enchant_id: other.enchant_id,
            azerite_power: other.azerite_power.clone(),
            azerite_level: other.azerite_level
        }
    }
}
