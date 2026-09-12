use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FMonth {
    fix: HashMap<String, f64>,
    food: HashMap<String, f64>,
    freetime: HashMap<String, f64>,
    savings: HashMap<String, f64>,
    total: f64,
}

impl FMonth {

    pub fn new() -> Self {
        Self {
            fix: HashMap::new(),
            food: HashMap::new(),
            freetime: HashMap::new(),
            savings: HashMap::new(),
            total: 0.0,
        }
    }


    pub fn keys(&self) -> [&str; 4] {
        ["fix", "food", "freetime", "savings"]
    }

    // pub fn extend(&mut self, what: &str, value: HashMap<String, f64>) {
    //     self.add_to_total(&value);
    //     match what {
    //         "fix" => { self.fix.extend(value); }
    //         "food" => { self.food.extend(value); }
    //         "freetime" => { self.freetime.extend(value); }
    //         "savings" => {  self.savings.extend(value); }
    //         _ => { println!("invalid command") }
    //     }
    // }

    pub fn insert(&mut self, what: &str, key: String, value: f64) {
        self.add_to_total(&HashMap::from([(key.clone(), value)]));
        match what {
            "fix" => { self.fix.entry(key).and_modify(|elem| *elem += value).or_insert(value); }
            "food" => { self.food.entry(key).and_modify(|elem| *elem += value).or_insert(value); }
            "freetime" => { self.freetime.entry(key).and_modify(|elem| *elem += value).or_insert(value); }
            "savings" => {  self.savings.entry(key).and_modify(|elem| *elem += value).or_insert(value); }
            _ => { println!("invalid command") }
        }
    }
    // pub fn extend_fix(&mut self, value: HashMap<String, f64>) {
    //     self.add_to_total(&value);
    //     self.fix.extend(value);
    // }
    //
    // pub fn insert_fix(&mut self, key: String, value: f64) {
    //     self.fix.insert(key.clone(), value);
    //     self.add_to_total(&HashMap::from([(key, value)]));
    // }
    //
    // pub fn extend_food(&mut self, value: HashMap<String, f64>) {
    //     self.add_to_total(&value);
    //     self.fix.extend(value);
    // }
    //
    // pub fn insert_food(&mut self, key: String, value: f64) {
    //     self.fix.insert(key.clone(), value);
    //     self.add_to_total(&HashMap::from([(key, value)]));
    // }
    //
    // pub fn extend_free_time(&mut self, value: HashMap<String, f64>) {
    //     self.add_to_total(&value);
    //     self.fix.extend(value);
    // }
    //
    // pub fn insert_free_time(&mut self, key: String, value: f64) {
    //     self.fix.insert(key.clone(), value);
    //     self.add_to_total(&HashMap::from([(key, value)]));
    // }
    //
    // pub fn extend_savings(&mut self, value: HashMap<String, f64>) {
    //     self.add_to_total(&value);
    //     self.fix.extend(value);
    // }
    //
    // pub fn insert_savings(&mut self, key: String, value: f64) {
    //     self.fix.insert(key.clone(), value);
    //     self.add_to_total(&HashMap::from([(key, value)]));
    // }

    fn add_to_total(&mut self, to_add: &HashMap<String, f64>) {
        for v in to_add.values() {
            self.total += v;
        }
    }

}
