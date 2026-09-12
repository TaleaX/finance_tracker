use std::process;

pub fn shutdown(exit_code: i32) {
    process::exit(exit_code);
}
