use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn day_1() {
    // File hosts must exist in current path before this produces output
    let mut first = false;
    let mut previous = 0;
    let mut total_increase = 0;
    if let Ok(lines) = read_lines("./day_1_input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let value_as_int = ip.parse::<i32>().unwrap();
                if first == false {
                    first = true;
                    previous = value_as_int;
                }
                else if value_as_int > previous{
                    total_increase += 1;
                }
                previous = value_as_int;
            }
        }
    }
    print!("{}", total_increase)
}

fn main() {
    println!("Hello, world!");
    day_1();
}
