use std::io;
mod structures;
mod parser;
mod command;
mod shutdown;


use structures::month::FMonth;
// use std::env;
use chrono::Datelike;
use command::exec_cmd;


// fn is_directory_empty(directory: &str) -> std::io::Result<bool> {
//     let mut entries = fs::read_dir(directory)?;
//     Ok(entries.next().is_none())
// }

// fn read_json(path_file: &str) -> Result<String, std::io::Error> {
//     Ok(fs::read_to_string(path_file)?)
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
    let mut month = FMonth::from_json(&path_db);//FMonth::new();
    let cmds = ["view", "update", "insert", "add", "sub" ,"exit"];
    loop {
        println!("Enter Command: {:?}", cmds);
        inp.clear();
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read line");
        let cmd: &str = inp.trim();
        if cmds.contains(&cmd) {
            let _ = exec_cmd(&cmd, &mut month, &path_db);
        } else {
            println!("invalid command");
        }
    }
}

