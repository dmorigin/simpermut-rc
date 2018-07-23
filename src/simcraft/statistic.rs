

use item::Item;
use std::cell::RefCell;
use configuration::Configuration;


#[derive(Default)]
pub struct Data {
    item_id: u32,
    avg_dps: f32,
    dps_value: Vec<f32>,
    rating: i32
}

impl Data {
    pub fn new(id: u32) -> Data {
        Data {
            item_id: id,
            avg_dps: 0.0,
            dps_value: Vec::new(),
            rating: 0
        }
    }
}


pub struct Statistic
{
    pub items: RefCell<Vec<Data>>,
    pub tolerance: f32,
    pub threshold: i32
}


impl Statistic {
    pub fn new(config: &Configuration) -> Statistic {
        Statistic {
            items: RefCell::new(Vec::new()),
            tolerance: config.statistic.tolerance,
            threshold: config.statistic.threshold
        }
    }

    pub fn update(&self, stack: &[Item], dps: f32, min_dps: f32, max_dps: f32) {
        for i in stack.iter() {
            self._add_new_one(i);
        }

        // rate all items
        for data in self.items.borrow_mut().iter_mut() {
            self._rate_item(data, dps, min_dps, max_dps);
        }
    }

    pub fn has_ignores(&self, stack: &[Item]) -> bool {
        for i in stack.iter() {
            for d in self.items.borrow().iter() {
                if i.id == d.item_id && d.rating <= self.threshold {
                    return true;
                }
            }
        }

        false
    }

    pub fn ignore(&self, item: &Item) -> bool {
        for d in self.items.borrow().iter() {
            if d.item_id == item.id && d.rating <= self.threshold {
                return true;
            }
        }

        false
    }

    // add a new item to the directory
    fn _add_new_one(&self, item: &Item) {
        // check for existing one
        for data in self.items.borrow().iter() {
            if data.item_id == item.id {
                return;
            }
        }

        // add new one
        self.items.borrow_mut().push(Data::new(item.id));
    }

    fn _rate_item(&self, data: &mut Data, dps: f32, min_dps: f32, max_dps: f32) {
        // cal avg dps
        data.avg_dps = Statistic::_calc_avg_dps(&mut data.dps_value, dps);

        // 10% below min dps
        let min = min_dps / (1.0 + (self.tolerance / 100.0));

        data.rating = ((data.avg_dps - min) / (max_dps - min) * 100.0) as i32;
//        println!("Rating: Item({}) -> Rating({}) / DPS({} / {}) Avg DPS({}) | Min({}), Tolerance({})", 
//            data.item_id, data.rating, min_dps, max_dps, data.avg_dps, min, self.tolerance);
    }

    fn _calc_avg_dps(range: &mut Vec<f32>, dps: f32) -> f32 {
        range.push(dps);

        let mut avg: f32 = 0.0;
        for i in range.iter() {
            avg += i;
        }

        avg / (range.len() as f32)
    }
}