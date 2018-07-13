

use slot::Slot;


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
}
