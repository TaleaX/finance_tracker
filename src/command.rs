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

#[derive(PartialEq, Debug, Copy, Clone)]
enum LoopOperation {
    CONTINUE,
    BREAK,
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

fn print_options(month: &mut FMonth) {
    println!("Command Options:\n \
    {:?} \n \
    q = quit to main \n \
    w = save changes to file \n \
    wq = save changes and quit to main \n \
    exit = exit this program ", month.keys());
}

fn get_inp(inp: &mut String) {
    inp.clear();
    io::stdin()
        .read_line(inp)
        .expect("Failed to read line");
}


fn check_cmd(inp: &str, file_path: &str, out: &String) -> LoopOperation {
    match inp {
        "w"=> { fs::write(file_path, out).expect("Couldnt write to file"); LoopOperation::CONTINUE }
        "wq"=> { fs::write(file_path, out).expect("Couldnt write to file"); LoopOperation::BREAK }
        "q"=> { LoopOperation::BREAK }
        "exit"=> { shutdown(0); LoopOperation::BREAK }
        _ => { println!("Invalid Command!"); LoopOperation::CONTINUE }
    }
}


fn change(month: &mut FMonth, file_path: &str, cmd_type: CmdType) -> Result<(), Box<dyn std::error::Error>> {
    let mut inp = String::new();
    let mut out = String::new();
    loop {
        print_options(month);
        get_inp(&mut inp);
        let mkey: String = inp.trim().to_string();
        if check_cmd(&inp.trim(), file_path, &out) == LoopOperation::BREAK {
            break; 
        }

        println!("Inserting into {}", &mkey);
        print!("Use Pattern key,val;key2,val2...\n>> ");
        std::io::stdout().flush().unwrap();
        get_inp(&mut inp);
        if check_cmd(&inp.trim(), file_path, &out) == LoopOperation::BREAK { 
            break; 
        }

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
        out = serde_json::to_string(&month).expect("serialization failed");
    }
    Ok(())
}
