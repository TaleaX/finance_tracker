mod structures;
mod parser;
mod cli;
mod shutdown;


use structures::month::FMonth;
// use std::env;
use chrono::Datelike;
use cli::command::exec_cmd;
use cli::input::{get_inp, print_options, InputOptions};


// fn is_directory_empty(directory: &str) -> std::io::Result<bool> {
//     let mut entries = fs::read_dir(directory)?;
//     Ok(entries.next().is_none())
// }


fn main() {
    // let args: Vec<String> = env::args().collect();
    let  current_date = chrono::Utc::now();
    // println!("{}", current_date.year());
    // println!("{}", current_date.month());
    // if args.len() <= 1 { shutdown(0); }
    // let path_db: String = args[1].trim().to_string();
    let path_db = format!("/home/tdehne/projects/self/rust/finance_tracker_db/2026/{}.json", current_date.month());

    let mut inp = String::new();
    let mut month = FMonth::from_json(&path_db);
    let cmds = ["view", "update", "insert", "add", "sub" ,"exit"];
    loop {
        print_options(&mut month, InputOptions::Main);
        get_inp(&mut inp);
        let _ = exec_cmd(inp.trim(), &mut month, &path_db);
    }
}

