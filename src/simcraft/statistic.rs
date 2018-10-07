

use item::Item;
use std::cell::RefCell;
use configuration::Configuration;


#[derive(Default)]
pub struct Data {
    item_id: u32,
    dps_avg: f32,
    dps_range: Vec<f32>,
//    dps_range: f64,
    rating: i32,
    seen: i32
}

impl Data {
    pub fn new(id: u32, dps: f32) -> Data {
        let mut data = Data {
            item_id: id,
            dps_avg: dps,
//            dps_range: dps as f64,
            dps_range: Vec::new(),
            rating: 0,
            seen: 1
        };

        data.dps_range.push(dps);

        data
    }
}


pub struct Statistic
{
    pub items: RefCell<Vec<Data>>,
    pub tolerance: f32,
    pub threshold: i32,
    pub iterations: u64,
    pub range_size: usize,
    pub modifier: f32
}


impl Statistic {
    pub fn new(config: &Configuration, iterations: u64, total_items: usize) -> Statistic {
        let mut obj = Statistic {
            items: RefCell::new(Vec::new()),
            tolerance: config.statistic.tolerance,
            threshold: config.statistic.threshold,
            iterations,
            range_size: config.simcraft.best_of,
            modifier: 0.0f32
        };

        obj.modifier = ((iterations as f64).sqrt() / (total_items as f64 / 16.0)) as f32;
        println!("Using modifier: {}", obj.modifier);

        obj
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
/*
    pub fn ignore(&self, item: &Item) -> bool {
        for d in self.items.borrow().iter() {
            if d.item_id == item.id && d.rating <= self.threshold {
                return true;
            }
        }

        false
    }
*/
    // add a new item to the directory
    fn _add_new_one(&self, item: &Item, dps: f32) {
        // check for existing one
        for data in self.items.borrow_mut().iter_mut() {
            if data.item_id == item.id {
                // calcluate avg dps
                self._calc_avg_dps(data, dps);
                return;
            }
        }

        // add new one
        self.items.borrow_mut().push(Data::new(item.id, dps));
    }

    fn _rate_item(&self, data: &mut Data, _dps: f32, min_dps: f32, max_dps: f32) {
        // could be negativ. So, we need more then the minimum dps.
        let mut tolerance = (1.0 + (self.tolerance / 100.0)) + (1.0 - (data.seen as f32 / self.modifier)).sin();
        if tolerance < 1.0 {
            tolerance = 1.0;
        }

        // calc minimum dps with tolerance
        let min = min_dps / tolerance;

        // rate item
        data.rating = ((data.dps_avg - min) / (max_dps - min) * 100.0) as i32;
        println!("Rating: Item({}) -> Rating({}) / DPS({} / {}) Avg DPS({}) | Min({}), Seen({}), Tol({})", 
            data.item_id, data.rating, min_dps, max_dps, data.dps_avg, min, data.seen, tolerance);
    }

/*
    fn _calc_avg_dps(&self, data: &mut Data, dps: f32) {
        // one more seen
        data.seen += 1;

        // add to dps container
        data.dps_range += dps as f64;

        // calc avg dps
        data.dps_avg = (data.dps_range / data.seen as f64) as f32;
    }
*/

    fn _calc_avg_dps(&self, data: &mut Data, dps: f32) {
        data.seen += 1;

        data.dps_range.push(dps);
        data.dps_range.sort_by(|a, b| b.partial_cmp(a).unwrap());
        
        if data.dps_range.len() > self.range_size {
            data.dps_range.pop();
        }

        let mut sum: f32 = 0.0;
        for i in data.dps_range.iter() {
            sum += i;
        }

        data.dps_avg = sum / (data.dps_range.len() as f32);
    }

}