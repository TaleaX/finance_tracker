use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs;
use std::collections::hash_map::Entry;
use crate::cli::command::CmdType;


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

    pub fn from_json(path_file: &str) -> Self {
        let data_str: String = match fs::read_to_string(path_file) {
            Ok(data) => data,
            Err(_error) => {
                println!("Couldn't read from {path_file} ... and empty object will be initialized");
                return Self::new();
            }
        };
        let month: Self = serde_json::from_str(&data_str).expect("{data_str} must be a valid json string!!");
        month
    }

    pub fn keys(&self) -> [&str; 4] {
        ["fix", "food", "freetime", "savings"]
    }

    pub fn update(&mut self, what: &str, key: String, value: f64) {
        match what {
            "fix" => { self.total += modify_entry(&mut self.fix, key, value, CmdType::UPDATE); }
            "food" => { self.total += modify_entry(&mut self.food, key, value, CmdType::UPDATE);}
            "freetime" => { self.total += modify_entry(&mut self.freetime, key, value, CmdType::UPDATE); }
            "savings" => {  self.total += modify_entry(&mut self.savings, key, value, CmdType::UPDATE); }
            _ => { println!("invalid key") }
        }
    }

    pub fn insert(&mut self, category: &str, key: String, value: f64) {
        let diff: f64 = match category {
                            "fix" => {
                                match self.fix.entry(key) {
                                        Entry::Occupied(entry) => {
                                            println!("{} fixed entry already exists\n \
                                                Since its a fixed entry you cannot add to it, \
                                                to change it please use the 'update' command instead", entry.get());
                                            0.0
                                        }

                                        Entry::Vacant(entry) => { entry.insert(value); value }
                                }
                            }
                            "food" => { self.food.entry(key).and_modify(|elem| *elem += value).or_insert(value); value }
                            "freetime" => { self.freetime.entry(key).and_modify(|elem| *elem += value).or_insert(value); value }
                            "savings" => {  self.savings.entry(key).and_modify(|elem| *elem += value).or_insert(value); value }
                            _ => { println!("invalid command"); 0.0 }
                        };
        self.total += diff;
    }

    pub fn delete(&mut self, category: &str, key: String) {
        match category {
            "fix" => { self.fix.remove(&key); }
            "food" => { self.food.remove(&key); }
            "freetime" => { self.freetime.remove(&key); }
            "savings" => {  self.savings.remove(&key); }
            _ => { println!("invalid command"); }
        }
    }

    pub fn add(&mut self, category: &str, key: String, value: f64) {
        match category {
            "fix" => { self.total += modify_entry(&mut self.fix, key, value, CmdType::ADD); }
            "food" => { self.total += modify_entry(&mut self.food, key, value, CmdType::ADD); }
            "freetime" => { self.total += modify_entry(&mut self.freetime, key, value, CmdType::ADD); }
            "savings" => { self.total += modify_entry(&mut self.savings, key, value, CmdType::ADD); }
            _ => { println!("invalid command"); }
        }
    }

    pub fn sub(&mut self, category: &str, key: String, value: f64) {
        match category {
            "fix" => { self.total += modify_entry(&mut self.fix, key, value, CmdType::SUB); }
            "food" => { self.total += modify_entry(&mut self.food, key, value, CmdType::SUB); }
            "freetime" => { self.total += modify_entry(&mut self.freetime, key, value, CmdType::SUB); }
            "savings" => { self.total += modify_entry(&mut self.savings, key, value, CmdType::SUB); }
            _ => { println!("invalid command"); }
        }
    }

}


// fn get_diff (attr: Option<&f64>, new_val: f64) -> f64 {
//     let old_val: f64 = match attr {
//         Some(attr) => *attr,
//         None => 0.0
//     };
//     new_val - old_val
// }


fn modify_entry(map: &mut HashMap<String, f64>, key: String, value: f64, operation: CmdType) -> f64 {
    if let Some(mvalue) = map.get_mut(&key) {
        let tmp: f64 = *mvalue;
        match operation {
            CmdType::ADD => { *mvalue += value; }
            CmdType::SUB => { *mvalue -= value; }
            CmdType::UPDATE => { *mvalue = value; }
            _ => { println!("Operations: {:?} doesn't exist!", operation) }
        }
        return *mvalue - tmp;
    } else {
        println!("{:?} doesn't exist!\nInsert it first!", key);
    }
    0.0
}

