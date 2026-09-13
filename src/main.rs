mod structures;
mod parser;
mod cli;
mod shutdown;


use std::collections::HashMap;
use structures::month::FMonth;
use std::env;
use chrono::Datelike;
use cli::command::exec_cmd;
use cli::input::{get_inp, print_options, InputOptions};

fn main() {
    let  current_date = chrono::Utc::now();
    let env_vars: HashMap<String, String> = env::vars().collect();
    let mut path_db: String = format!("{}.json", current_date.month());
    if let Some(val) = env_vars.get("FTRACKER_DB") {
        path_db = format!("{}/{}.json", val, current_date.month());
    }

    let mut inp = String::new();
    let mut month = FMonth::from_json(&path_db);
    loop {
        print_options(&mut month, InputOptions::Main);
        get_inp(&mut inp);
        let _ = exec_cmd(inp.trim(), &mut month, &path_db);
    }
}

