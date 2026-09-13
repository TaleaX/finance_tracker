use crate::shutdown::shutdown;
use crate::parser::parse_input;
use std::fs;
use crate::structures::month::FMonth;
use crate::cli::input::{print_options, get_inp, InputOptions};
use LoopOperation::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum CmdType {
    UPDATE,
    INSERT,
    ADD,
    SUB,
    DELETE,
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum LoopOperation {
    Continue,
    Break,
    Ignore,
}

pub fn exec_cmd(cmd: &str, month: &mut FMonth, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        "view" => println!("{:#?}", month),
        "insert" => change(month, &file_path, CmdType::INSERT)?,
        "update" => change(month, &file_path, CmdType::UPDATE)?,
        "add" => change(month, &file_path, CmdType::ADD)?,
        "sub" => change(month, &file_path, CmdType::SUB)?,
        "delete" => change(month, &file_path, CmdType::DELETE)?,
        "exit" => shutdown(0),
        _ => println!("Invalid Command!"),
    }
    Ok(())
}

fn check_cmd(inp: &str, file_path: &str, out: &String) -> LoopOperation {
    match inp {
        "w" => { fs::write(file_path, out).expect("Couldnt write to file"); Continue }
        "wq" => { fs::write(file_path, out).expect("Couldnt write to file"); Break }
        "q" => Break,
        "qi" => Continue,
        "exit" => { shutdown(0); Break }
        _ => Ignore,
    }
}


fn change(month: &mut FMonth, file_path: &str, cmd_type: CmdType) -> Result<(), Box<dyn std::error::Error>> {
    let mut inp = String::new();
    let mut out = String::new();
    loop {
        print_options(month, InputOptions::InpKey);
        get_inp(&mut inp);
        let mkey: String = inp.trim().to_string();
        let mut loop_operation = check_cmd(&inp.trim(), file_path, &out);

        if month.keys().contains(&mkey.as_str()) {
            println!("Modifying: {}", &mkey);
            print_options(month, InputOptions::InpVal);
            get_inp(&mut inp);

            loop_operation = check_cmd(&inp.trim(), file_path, &out);
            if loop_operation == Break { break; } 
            else if loop_operation == Continue { continue; }

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
                } else if cmd_type == CmdType::DELETE {
                    month.delete(&mkey, elem.0);
                }
            }
        } else if loop_operation == Break { 
            break; 
        } else if loop_operation == Ignore {
            println!("Invalid Command!");
        }
        out = serde_json::to_string(&month).expect("serialization failed");
    }
    Ok(())
}
