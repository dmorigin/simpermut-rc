

use item::Item;
use std::cell::RefCell;
use configuration::Configuration;


#[derive(Default)]
pub struct Data {
    item_id: u32,
    dps_avg: f32,
//    dps_range: Vec<f32>,
    dps_range: f64,
    rating: i32,
    seen: i32
}

impl Data {
    pub fn new(id: u32, dps: f32) -> Data {
        Data {
            item_id: id,
            dps_avg: dps,
            dps_range: dps as f64,
//            dps_range: Vec::new(),
            rating: 0,
            seen: 1
        }
    }
}


pub struct Statistic
{
    pub items: RefCell<Vec<Data>>,
    pub tolerance: f32,
    pub threshold: i32,
    pub iterations: u64
}


impl Statistic {
    pub fn new(config: &Configuration, iterations: u64) -> Statistic {
        Statistic {
            items: RefCell::new(Vec::new()),
            tolerance: config.statistic.tolerance,
            threshold: config.statistic.threshold,
            iterations
        }
    }

    pub fn update(&self, stack: &[Item], dps: f32, min_dps: f32, max_dps: f32) {
        for i in stack.iter() {
            self._add_new_one(i, dps);
        }

        // rate all items
        for data in self.items.borrow_mut().iter_mut() {
            self._rate_item(data, dps, min_dps, max_dps);
        }
    }

    pub fn has_ignores(&self, stack: &[Item]) -> bool {
        for i in stack.iter() {
            for d in self.items.borrow().iter() {
                if i.id == d.item_id && d.rating < self.threshold {
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
    fn _add_new_one(&self, item: &Item, dps: f32) {
        // check for existing one
        for data in self.items.borrow_mut().iter_mut() {
            if data.item_id == item.id {
                // calcluate avg dps
                Statistic::_calc_avg_dps(data, dps);
                return;
            }
        }

        // add new one
        self.items.borrow_mut().push(Data::new(item.id, dps));
    }

    fn _rate_item(&self, data: &mut Data, _dps: f32, min_dps: f32, max_dps: f32) {
        // calc minimum dps with tolerance
        let min = min_dps / (1.0 + (self.tolerance / 100.0));

        // rate item
        data.rating = ((data.dps_avg - min) / (max_dps - min) * 100.0) as i32;
        println!("Rating: Item({}) -> Rating({}) / DPS({} / {}) Avg DPS({}) | Min({}), Seen({})", 
            data.item_id, data.rating, min_dps, max_dps, data.dps_avg, min, data.seen);
    }

    fn _calc_avg_dps(data: &mut Data, dps: f32) {
        // one more seen
        data.seen += 1;

        // add to dps container
        data.dps_range += dps as f64;

        // calc avg dps
        data.dps_avg = (data.dps_range / data.seen as f64) as f32;
    }

    fn _old_calc_avg_dps(range: &mut Vec<f32>, dps: f32) -> f32 {
        range.push(dps);

        let mut avg: f32 = 0.0;
        for i in range.iter() {
            avg += i;
        }

        avg / (range.len() as f32)
    }
}