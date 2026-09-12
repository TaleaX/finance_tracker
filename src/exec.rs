use crate::shutdown::shutdown;
use crate::parser::parse_input;
use std::fs;
use std::io;
use crate::structures::month::FMonth;

pub fn exec_cmd(cmd: &str, month: &mut FMonth, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if cmd == "update" { println!("update"); }
    else if cmd == "view" { println!("view"); }
    else if cmd == "insert" { insert(month, &file_path)?; }
    else if cmd == "exit" { shutdown(0); }
    Ok(())
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
