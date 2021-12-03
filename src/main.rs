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
    if let Ok(lines) = read_lines("./day_1_input.txt") {
        let mut first = false;
        let mut previous = 0;
        let mut total_increase = 0;
    
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
        println!("{}", total_increase)
    }

}

fn day_2() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./day_1_input.txt") {
        // Consumes the iterator, returns an (Optional) String

        let mut total_increase = 0;

        let mut all_numbers = vec![];
        for line in lines {
            if let Ok(ip) = line {
                let value_as_int = ip.parse::<i32>().unwrap();
                all_numbers.push(value_as_int);
            }
        }

        let mapped_values : Vec<i32> = all_numbers.windows(3).into_iter().map(|a|{
            return a[0] + a[1] + a[2];
        }).collect();
        let mut previous = mapped_values[0];
        for value in mapped_values {
            if value > previous {
                total_increase += 1;
            }
            previous = value;
        }

        println!("{}", total_increase)
    }

}

fn main() {
    day_1();
    day_2();
}
