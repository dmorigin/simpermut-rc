
use slot::Slot;
use item::Item;
use std::slice::{Iter, IterMut};
use std::ops::{Index, IndexMut};


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
                slot: *slot,
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

    pub fn index(&self, slot: &Slot) -> Option<usize> {
        let mut i: usize = 0;

        for iter in self.list.iter() {
            if iter.slot == *slot {
                return Some(i);
            }

            i = i + 1;
        }

        return None;
    }

    pub fn push(&self, slot: &Slot, item: &Item) {
        // search for existiing one
        for iter in self.list.iter() {
            if iter.slot == *slot {
                iter.items.push(*item);
                return;
            }
        }

        // insert new one
        let mut entry = pair::Pair::new(slot);
        entry.items.push(*item);
        self.list.push(entry);
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn iter(&self) -> Iter<'_, pair::Pair> {
        self.list.iter()
    }

    pub fn iter_mut(&self) -> IterMut<'_, pair::Pair> {
        self.list.iter_mut()
    }
}

impl Index<usize> for ItemMap {
    type Output = pair::Pair;

    fn index(&self, index: usize) -> &pair::Pair {
        &self.list[index]
    }
}

impl IndexMut<usize> for ItemMap {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut pair::Pair {
        &mut self.list[index]
    }
}
