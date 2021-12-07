use std::collections::btree_map::Range;
use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead, Lines, BufReader};
use std::path::Path;

use std::cmp::PartialEq;
use std::ops::BitXor;
use std::ops::Shl;
use rayon::prelude::*;

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
    if let Ok(mut lines) = read_lines("./day_1_input.txt") {
        //Grab the first line and set up our control variable the loop
        let mut  previous = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
        let mut total_increase = 0;
        for line in lines {
            if let Ok(ip) = line {
                let value_as_int = ip.parse::<i32>().unwrap();
                if value_as_int > previous{
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
    if let Ok(mut lines) = read_lines("./day_1_input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut total_increase = 0;
        let a = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
        let mut b = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
        let mut c = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
        
        let mut previous = a + b + c;

        for line in lines {
            let new_number = line.unwrap().parse::<i32>().unwrap();
            let value = b + c + new_number;
            if value > previous {
                total_increase += 1;
            }
            b = c;
            c = new_number;
            previous = value;
        }
        println!("{}", total_increase);
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


fn read_board(file: &mut Lines<BufReader<File>>) -> Vec<(u32, bool)> {
    let mut values = vec![];
    for i in 0..5 {
        let line: Vec<(u32, bool)> = file.next().unwrap().unwrap().split_ascii_whitespace().map(|x|x.parse().unwrap()).map(|x|return (x, false)).collect();
        values.extend(line);
    }
    return values;
}


fn has_board_been_completed(board: &Vec<(u32, bool)>) -> Option<u32> {
    for x in 0..5 {
        let mut count = 0;
        for y in 0..5 {
            let index = y * 5 + x;
            if board[index].1 {
                count += 1;
            }
        }
        if count == 5 {
            return Some(board.iter().fold(0, |sum, x|{ if x.1 == false{
                return sum + x.0;
            }else {sum}}));
        }
    }

    for x in 0..5 {
        let mut count = 0;
        for y in 0..5 {
            let index = x * 5 + y;
            if board[index].1 {
                count += 1;
            }
        }
        if count == 5 {
            return Some(board.iter().fold(0, |sum, x|{ if x.1 == false{
                return sum + x.0;
            }else {sum}}));
        }
    }

    return None;
}

fn day_4() {
    println!("Done with day 4");
    if let Ok(mut lines) = read_lines("./day_4_input.txt") {
        let mut winning_boards = HashMap::new();
        let mut winning_order = vec![];
        let marks: Vec<u32> = lines.next().unwrap().unwrap().split(',').map(|x|x.parse().unwrap()).collect();

        let mut boards = vec![];
        while lines.next().is_some() {
            let value = read_board(&mut lines);
            boards.push(value);
        }
        

        for mark in marks {
            for (index, board) in boards.iter_mut().enumerate() {
                for cell in board.iter_mut() {
                    if cell.0 == mark {
                        cell.1 = true;
                    }
                }
                let result = has_board_been_completed(board);
                match result {
                    Some(value) => {
                        if winning_boards.contains_key(&index) == false {
                            winning_boards.insert(index, value * mark);
                            winning_order.push(index);
                        }
                    },
                    _ => {
    
                    }
                }            
            }
        }

        println!("{:?}", winning_boards[&winning_order[winning_order.len() - 1]]);
    }
    println!("Done with day 4");
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Point  {
    pub fn point_between(&self, other: &Point) -> Vec<Point> {
        let x_dir;

        if self.x == other.x {
            x_dir = 0;
        }
        else {
            if self.x > other.x {
                x_dir = -1;
            }
            else {
                x_dir = 1;
            }
        }

        let y_dir;
        if self.y == other.y {
            y_dir = 0;
        }
        else {
            if self.y > other.y {
                y_dir = -1;
            }
            else {
                y_dir = 1;
            }
        }
        let mut color_points = vec![];
        let mut start_point = Point{x: self.x, y:self.y};
        color_points.push(start_point);
        while start_point != *other {
            start_point.x += x_dir;
            start_point.y += y_dir;
            color_points.push(start_point);
        }
        if self.x == 6 && self.y == 4 {
            println!("{:?}", color_points);
        }
        return color_points;
    }
}

fn day_5() {
    if let Ok(mut lines) = read_lines("./day_5_input.txt") {
        let mut board = HashMap::new();
        let mut pairs = vec![];
        for line in lines {
            let mut one_lines = vec![];
            let unwrapped_line = line.unwrap();
            let a_line = unwrapped_line.split("->");

            for x in a_line {
                let b = x.split(',');
                let mut poses: Vec<i32> = vec![];
                for sss in b {
                    poses.push(sss.trim().parse().unwrap());
                }
                one_lines.push(poses);
            }
            pairs.push(one_lines);
        }

        let valid_pairs: Vec<_> = pairs.iter().filter(|x|{
            if x[0][0] == x[1][0] || x[0][1] == x[1][1] {
                return true;
            }

            let slope = (x[1][1] - x[0][1]) / (x[1][0] - x[0][0]);
            if slope == 1 || slope == -1 {
                return true;
            }
            if x[0][0] == x[0][1] && x[1][0] == x[1][1]  {
                return true;
            }
            if x[0][0] == x[1][1] && x[0][1] == x[1][0] {
                return true;
            }
            return false;
        }).collect();

        valid_pairs.iter().for_each(|foo|{
            let x1 = foo[0][0];
            let y1 = foo[0][1];
            let point_1 = Point{x: x1, y: y1};
            let x2 = foo[1][0];
            let y2 = foo[1][1];
            let point_2 = Point{x: x2, y: y2};
            for point in point_1.point_between(&point_2) {
                let key = (point.x, point.y);
                if board.contains_key(&key) == false {
                    board.insert(key, 0);
                }

                *board.get_mut(&key).unwrap() += 1;
            }
        });
        let mut count = 0;
        for key in board.keys() {
            if board[key] > 1 {
                count += 1;
            }
        }
        println!("{:?}", count);

        for x in 0..10 {
            for y in 0..10 {
                if board.contains_key(&(y, x)) {
                    print!("{:?} ", board[&(y, x)]);
                }
                else {
                    print!(". ");
                }
            }
            println!("");
        }
    }
}

fn day_6() {
    if let Ok(mut lines) = read_lines("./day_6_input.txt") {
        let numbers_line_as_string = lines.next().unwrap().unwrap();
        let numbers : Vec<u8> = numbers_line_as_string.split(',').map(|x|x.parse().unwrap()).collect();
        let mut fish_counter_a: [u64; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
        let fish_counter_b: [u64; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];

        for num in numbers {
            fish_counter_a[num as usize] += 1;
        }
        let mut fish_counters = [fish_counter_a, fish_counter_b];
        for i in 0..256 {
            println!("Gen: {:?}", i);
            let read_index = i % 2;
            let write_index = 1 - (i % 2);
            fish_counters[write_index][8] = fish_counters[read_index][0];
            fish_counters[write_index][7] = fish_counters[read_index][8];
            fish_counters[write_index][6] = fish_counters[read_index][7] + fish_counters[read_index][0];
            fish_counters[write_index][5] = fish_counters[read_index][6];            
            fish_counters[write_index][4] = fish_counters[read_index][5];
            fish_counters[write_index][3] = fish_counters[read_index][4];            
            fish_counters[write_index][2] = fish_counters[read_index][3];            
            fish_counters[write_index][1] = fish_counters[read_index][2];
            fish_counters[write_index][0] = fish_counters[read_index][1];
        }
        let mut a_count = 0;
        let mut b_count = 0;
        for i in 0..9 {
            a_count += fish_counters[0][i];
            b_count += fish_counters[1][i];
        }
        println!("{:?}", a_count);
        println!("{:?}", b_count);
    }   
}

fn main() {
    day_6();
//    day_5();
    /*
    day_1_part_1();
    day_1_part_2();
    day_2_part_1();
    day_2_part_2();
    day_3();
    day_4();
    */
}
