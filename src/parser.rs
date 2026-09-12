pub fn parse_input(cmd: &str) -> Vec<(String, f64)> {
    let elems = cmd.trim().split(";");
    let mut lst: Vec<(String, f64)> = Vec::new();
    for elem in elems {
        let key_val: Vec<&str> = elem.trim().split(",").collect();
        println!("{:?}{:?}", key_val[0].trim(), key_val[1].trim());
        lst.push((key_val[0].trim().to_string(), key_val[1].trim().parse().unwrap()))
    }
    lst
}
