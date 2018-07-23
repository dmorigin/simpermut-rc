
use slot::{Slot, ESlot};
use item::Item;
use std::option::Option;


mod pair {
    use slot::Slot;
    use item::Item;
    use std::default::Default;


    pub struct Pair {
        pub slot: Slot,
        pub items: Vec<Item>,
    }

    impl Pair {
        pub fn new(slot: &Slot) -> Pair {
            Pair {
                slot: slot.clone(),
                items: Vec::new()
            }
        }
    }

    impl Default for Pair {
        fn default() -> Pair {
            Pair {
                slot: Slot::default(),
                items: Vec::default()
            }
        }
    }
}


pub struct ItemMap {
    list: Vec<pair::Pair>
}


impl ItemMap {
    pub fn new() -> ItemMap {
        ItemMap {
            list: Vec::new()
        }
    }

    pub fn push(&mut self, slot: &Slot, item: &Item) {
        // search for existiing one
        for iter in &mut self.list {
            if iter.slot == *slot {
                iter.items.push(item.clone());
                return;
            }
        }

        // insert new one
        let mut entry = pair::Pair::new(slot);
        entry.items.push(item.clone());
        self.list.push(entry);
    }

    pub fn get_slot(&self, pattern: ESlot) -> Option<&Vec<Item>> {
        for iter in &self.list {
            if iter.slot.slot == pattern {
                return Some(&iter.items);
            }
        }

        None
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }
}
