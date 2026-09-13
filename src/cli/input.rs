use crate::structures::month::FMonth;
use std::io;
use std::io::Write;
use InputOptions::*;

const MAIN_OPTIONS: &str = "\
Command Options:
view = view the database
insert = create a new entry in the databse
update = change the value of an entry
add = add x to a specific entry
sub = substract x from a specific entry
delete = delete an entry from the database
exit = exit this program
";

const MOD_OPTIONS: &str = "\
Command Options:
q = quit to main
qi = quit current input
w = save changes to file
wq = save changes and quit to main
exit = exit this program
<category> = enter one of the existing categories that you like to modify
";


const KEY_VAL_PATTERN: &str = "\
Use the following pattern to modify a certain key:value
key,val;key2,val2...
";

pub enum InputOptions {
    Main,
    InpKey,
    InpVal,
}

pub fn get_inp(inp: &mut String) {
    inp.clear();
    io::stdin()
        .read_line(inp)
        .expect("Failed to read line");
}

pub fn print_options(month: &mut FMonth, current_inp: InputOptions) {
    match current_inp {
        Main => println!("\n{}", MAIN_OPTIONS),
        InpKey => println!("\n{}\nCategories: {:?}", MOD_OPTIONS, month.keys()),
        InpVal => { print!("\n{}\n>>", KEY_VAL_PATTERN); std::io::stdout().flush().unwrap(); }
    }
}
