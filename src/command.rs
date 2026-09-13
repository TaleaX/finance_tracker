use crate::shutdown::shutdown;
use crate::parser::parse_input;
use std::fs;
use std::io;
use std::io::Write;
use crate::structures::month::FMonth;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum CmdType {
    UPDATE,
    INSERT,
    ADD,
    SUB,
    DELETE,
}

pub fn exec_cmd(cmd: &str, month: &mut FMonth, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if cmd == "view" { println!("{:#?}", month); }
    else if cmd == "insert" { change(month, &file_path, CmdType::INSERT)?; }
    else if cmd == "update" { change(month, &file_path, CmdType::UPDATE)?; }
    else if cmd == "add" { change(month, &file_path, CmdType::ADD)?; }
    else if cmd == "sub" { change(month, &file_path, CmdType::SUB)?; }
    else if cmd == "exit" { shutdown(0); }
    Ok(())
}

fn change(month: &mut FMonth, file_path: &str, cmd_type: CmdType) -> Result<(), Box<dyn std::error::Error>> {
    let mut inp = String::new();
    let mut out: String = String::new();
    loop {
        println!("Command Options:\n \
        {:?} \n \
        q = quit to main \n \
        w = save changes to file \n \
        wq = save changes and quit to main \n \
        exit = exit this program ", month.keys());
        inp.clear();
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read line");
        let mkey: String = inp.trim().to_string();
        if month.keys().contains(&mkey.as_str()) {
            println!("Inserting into {}", &mkey);
            print!("Use Pattern key,val;key2,val2...\n>> ");
            std::io::stdout().flush().unwrap();
            inp.clear();
            io::stdin()
                .read_line(&mut inp)
                .expect("Failed to read line");
            if inp.trim() == "exit" { shutdown(0); }
            let vec1 = parse_input(&inp);
            for elem in vec1 {
                if cmd_type == CmdType::INSERT {
                    month.insert(&mkey, elem.0, elem.1);
                } else if cmd_type == CmdType::UPDATE {
                    month.update(&mkey, elem.0, elem.1);
                } else if cmd_type == CmdType::ADD {
                    month.add(&mkey, elem.0, elem.1);
                } else if cmd_type == CmdType::SUB {
                    month.sub(&mkey, elem.0, elem.1);
                }
            }
            out = serde_json::to_string(&month).expect("serialization failed");
            // println!("{:#?}", out);
        }
        else if mkey == "w" { fs::write(file_path, &out)?; }
        else if mkey == "wq" { fs::write(file_path, &out)?;  break; }
        else if mkey == "q" { break; }
        else if mkey == "exit" { shutdown(0); }
        else { println!("invalid command"); }
    }
    Ok(())
}
