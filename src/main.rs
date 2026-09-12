use std::io;
mod structures;
use structures::month::FMonth;
use std::process;
// use std::env;
use std::fs;
use chrono::Datelike;
use std::path::Path;

fn shutdown(exit_code: i32) {
    process::exit(exit_code);
}

fn exec(cmd: &str, month: &mut FMonth, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if cmd == "update" { println!("update"); }
    else if cmd == "view" { println!("view"); }
    else if cmd == "insert" { insert(month, &file_path)?; }
    else if cmd == "exit" { shutdown(0); }
    Ok(())
}


fn parse_input(cmd: &str) -> Vec<(String, f64)> {
    let elems = cmd.trim().split(";");
    let mut lst: Vec<(String, f64)> = Vec::new();
    for elem in elems {
        let key_val: Vec<&str> = elem.trim().split(",").collect();
        println!("{:?}{:?}", key_val[0].trim(), key_val[1].trim());
        lst.push((key_val[0].trim().to_string(), key_val[1].trim().parse().unwrap()))
    }
    lst
}

fn insert(month: &mut FMonth, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut inp = String::new();
    let mut out: String = String::new();
    println!("{:#?}", month.keys());
    loop {
        inp.clear();
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read line");
        let mkey: String = inp.trim().to_string();
        if month.keys().contains(&mkey.as_str()) {
            inp.clear();
            io::stdin()
                .read_line(&mut inp)
                .expect("Failed to read line");
            if inp.trim() == "exit" { shutdown(0); }
            let vec1 = parse_input(&inp);
            for elem in vec1 {
                month.insert(&mkey, elem.0, elem.1);
            }
            out = serde_json::to_string(&month).expect("serialization failed");
            println!("{:#?}", out);
        }
        else if mkey == "w" { fs::write(file_path, &out)?; }
        else if mkey == "wq" { fs::write(file_path, &out)?;  break; }
        else if mkey == "q" { break; }
        else if mkey == "exit" { shutdown(0); }
        else { println!("invalid command"); }
    }
    Ok(())
}

// fn is_directory_empty(directory: &str) -> std::io::Result<bool> {
//     let mut entries = fs::read_dir(directory)?;
//     Ok(entries.next().is_none())
// }

fn read_json(path_file: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(fs::read_to_string(path_file)?)
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    let  current_date = chrono::Utc::now();
    // println!("{}", current_date.year());
    // println!("{}", current_date.month());
    // if args.len() <= 1 { shutdown(0); }
    // let path_db: String = args[1].trim().to_string();
    let path_db = format!("/home/tdehne/projects/self/rust/finance_tracker_db/2026/{}.json", current_date.month());

    // let mut m: FMonth = serde_json::from_str(&out.as_str()).unwrap();
    let mut inp = String::new();
    let mut month = FMonth::new();
    if Path::new(&path_db).exists() {
        month = serde_json::from_str(&read_json(&path_db).unwrap()).unwrap();
    }
    println!("{:#?}", month);
    let cmds = ["view", "update", "insert", "exit"];
    loop {
        println!("Enter Command: {:?}", cmds);
        inp.clear();
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read line");
        let cmd: &str = inp.trim();
        if cmds.contains(&cmd) {
            let _ = exec(&cmd, &mut month, &path_db);
        } else {
            println!("invalid command");
        }
    }
}

