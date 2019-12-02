use std::env;
use std::fs;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    day_2_exercise_1(args);

    // println!("{:?}", day_2_exercise_1(args));
}


/// --- Day 2, Exercise 1 ------------------------------------------------------
fn day_2_exercise_1(args: Vec<String>) {
    let filename = &args[1];
    let raw_program: String = fs::read_to_string(filename).expect("Error reading file");
    let mut program: Vec<usize> = raw_program
                                .trim()
                                .split(',')
                                .into_iter()
                                .map(|n| n.parse::<usize>())
                                .filter_map(Result::ok)
                                .collect();

    // "before running the program, replace position 1 with the value 12 
    // and replace position 2 with the value 2."
    program[1] = 12;
    program[2] = 2;

    let program_length = program.len();

    let mut i = 0;
    let mut opcode;
    let mut value_1;
    let mut value_2;
    let mut store_index;

    while i < program_length && program[i] != 99 {
        opcode = program[i];
        value_1 = program[program[i+1]];
        value_2 = program[program[i+2]];
        store_index = program[i+3];

        match opcode {
            1 => {
                program[store_index] = value_1 + value_2;
            },
            2 => {
                program[store_index] = value_1 * value_2;
            },
            99 => { break; },
            _ => panic!("Error"),
        }
        i += 4;
    }

    println!("{:?}", program);
}


/// --- Day 1, Exercise 2 ------------------------------------------------------
#[allow(dead_code)]
fn day_1_exercise_2(args: Vec<String>) -> i32 {
    let filename = &args[1];
    let file = fs::File::open(filename).expect("Error reading file");

    let reader = BufReader::new(file);

    let mut total_fuel = 0;

    for line in reader.lines() {
        let mass = line.unwrap().parse().unwrap();

        let fuel = recursively_calculate_fuel_for(mass);

        total_fuel += fuel;
    }

    total_fuel
}

fn recursively_calculate_fuel_for(n: i32) -> i32 {
    let fuel_for_n = (n as f64 / 3.0_f64).trunc() as i32 - 2;

    if fuel_for_n <= 0 {
        return 0;
    } else {
        return fuel_for_n + recursively_calculate_fuel_for(fuel_for_n);
    }
}


/// --- Day 1, Exercise 1 ------------------------------------------------------
#[allow(dead_code)]
fn day_1_exercise_1(args: Vec<String>) -> i32 {
    let filename = &args[1];
    let file = fs::File::open(filename).expect("Error reading file");

    let reader = BufReader::new(file);

    let mut total_fuel = 0;

    for line in reader.lines() {
        let mass: i32 = line.unwrap().parse().unwrap();

        let fuel = calculate_fuel_for(mass);

        total_fuel += fuel;
    }

    total_fuel
}

fn calculate_fuel_for(n: i32) -> i32 {
    (n as f64 / 3.0_f64).trunc() as i32 - 2
}

