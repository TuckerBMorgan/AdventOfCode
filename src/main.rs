use core::num;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::cmp::PartialEq;
use std::ops::BitXor;
use std::ops::Shl;
// From https://citizen-stig.github.io/2020/04/04/converting-bits-to-integers-in-rust-using-generics.html
fn convert<T: PartialEq + From<bool> + BitXor<Output = T> + Shl<Output = T> + Clone>(
    bits: &[T],
) -> T {
    let zero = T::from(false);
    let one = T::from(true);
    bits.iter()
        .filter(|&bit| bit == &zero.clone() || bit == &one.clone())
        .fold(zero.clone(), |result, bit| {
            (result << one.clone()) ^ bit.clone()
        })
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn day_1_part_1() {
    println!("Day 1 Part 1");
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

fn day_1_part_2() {
    println!("Day 1 Part 2");
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

fn day_2_part_1() {
    println!("Day 2 Part 1");
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./day_2_input.txt") {
        let mut horizontal_pos = 0;
        let mut depth = 0;

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let parts : Vec<_>= ip.split_ascii_whitespace().collect();
                if parts[0] == "forward" {
                    horizontal_pos += parts[1].parse::<i32>().unwrap()
                }
                else if parts[0] == "down" {
                    depth += parts[1].parse::<i32>().unwrap();
                }
                else {
                    depth -= parts[1].parse::<i32>().unwrap();
                }
            }
        }

        println!("{:?}", depth * horizontal_pos);
    }
}

fn day_2_part_2() {
    println!("Day 2 Part 2");
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./day_2_input.txt") {
        let mut horizontal_pos = 0;
        let mut depth = 0;
        let mut aim = 0;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let parts : Vec<_>= ip.split_ascii_whitespace().collect();
                if parts[0] == "forward" {
                    let value = parts[1].parse::<i32>().unwrap();
                    horizontal_pos += value;
                    depth += aim * value;
                }
                else if parts[0] == "down" {
                    aim += parts[1].parse::<i32>().unwrap();
                }
                else {
                    aim -= parts[1].parse::<i32>().unwrap();
                }
            }
        }

        println!("{:?}", depth * horizontal_pos);
    }
    else {
        println!("File not found");
    }
}

fn day_3() {
    println!("Day 3 Part 1");
    let number_size = 12;
    if let Ok(lines) = read_lines("./day_3_input.txt") {

        let mut values = vec![];
        let mut saved_lines = vec![];
        for _ in 0..number_size {
            values.push(vec![0, 0]);
        }

        for line in lines {
            if let Ok(ip) = line {                
                for i in 0..number_size {
                    let value = ip.chars().nth(i).unwrap().to_digit(10).unwrap();
                    values[i][value as usize] += 1;

                }
                saved_lines.push(ip);
            }
        }

        let mut gamma = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        for i in 0..number_size {
            if values[i][0] > values[i][1] {
                gamma[i] = 0;
            }
            else {
                gamma[i] = 1;
            }
        }

        let gamma_value = convert(&gamma);

        for i in 0..number_size {
            if gamma[i] == 0 {
                gamma[i] = 1;
            }
            else {
                gamma[i] = 0;
            }
        }

        let epsilon_value = convert(&gamma);

        println!("{:?}", gamma_value);
        println!("{:?}", epsilon_value);
        println!("{:?}", gamma_value * epsilon_value);

        let mut oxy_value = 0;
        let mut considering_indexes : Vec<usize> = (0..saved_lines.len()).collect();

        for i in 0..number_size {


            let common = considering_indexes.iter().fold(0, |sum, x|{
                if saved_lines[*x].chars().nth(i).unwrap().to_digit(10).unwrap() == 0 {
                    return sum - 1;
                }
                else {
                    sum + 1
                }
            });

            let mut most_common_value = 1;
            if common < 0 {
                most_common_value = 0;
            }

            considering_indexes = considering_indexes.iter().map(|x|*x).filter(
                |x| {
                    return saved_lines[*x].chars().nth(i).unwrap().to_digit(10).unwrap() == most_common_value;
                }
            ).collect();

            if considering_indexes.len() == 1 {
                let test : Vec<u32> = saved_lines[considering_indexes[0]].chars().map(|x|{
                    return x.to_digit(10).unwrap();
                }).collect();
                oxy_value = convert(&test);
                break;
            }
        }

        let mut co2_level = 0;
        let mut considering_indexes : Vec<usize> = (0..saved_lines.len()).collect();
        for i in 0..number_size {

            let common = considering_indexes.iter().fold(0, |sum, x|{
                if saved_lines[*x].chars().nth(i).unwrap().to_digit(10).unwrap() == 0 {
                    return sum - 1;
                }
                else {
                    sum + 1
                }
            });

            let mut least_common_value = 0;
            if common < 0 {
                least_common_value = 1;
            }

            considering_indexes = considering_indexes.iter().map(|x|*x).filter(
                |x| {
                    return saved_lines[*x].chars().nth(i).unwrap().to_digit(10).unwrap() == least_common_value;
                }
            ).collect();

            if considering_indexes.len() == 1 {
                let test : Vec<u32> = saved_lines[considering_indexes[0]].chars().map(|x|{
                    return x.to_digit(10).unwrap();
                }).collect();

                co2_level = convert(&test);
                break;
            }
        }
        println!("Day 3 Part 3");
        println!("{:?}", oxy_value);
        println!("{:?}", co2_level);
        println!("{:?}", oxy_value * co2_level);
    }
}

fn main() {
    day_1_part_1();
    day_1_part_2();
    day_2_part_1();
    day_2_part_2();
    day_3();
}
