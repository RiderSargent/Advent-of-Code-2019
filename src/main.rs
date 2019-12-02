use std::env;
use std::fs;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = fs::File::open(filename).expect("Error reading file");

    let reader = BufReader::new(file);

    let mut total_fuel = 0;

    for line in reader.lines() {
        let mass = line.unwrap().parse().unwrap();

        let fuel = calculate_fuel_for(mass);

        total_fuel += fuel;
    }

    println!("{:?}", total_fuel);
}

fn calculate_fuel_for(n: i32) -> i32 {
    let fuel_for_n = (n as f64 / 3.0_f64).trunc() as i32 - 2;

    if fuel_for_n <= 0 {
        return 0;
    } else {
        return fuel_for_n + calculate_fuel_for(fuel_for_n);
    }
}
