use std::fs;
use std::io;

pub fn load_program(filename: &str) -> Vec<i32> {
    let raw_program: String = fs::read_to_string(filename).expect("Error reading file");
    let program: Vec<i32> = raw_program
        .trim()
        .split(',')
        .into_iter()
        .map(|n| n.parse::<i32>())
        .filter_map(Result::ok)
        .collect();

    program
}

pub fn run_program(mut program: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let program_length = program.len();

    while i < program_length && program[i] != 99 {
        let offset: usize;
        match program[i] {
            1 => {
                // add - opcode, v1, v2, store_index
                let i_1: usize = program[i + 1] as usize;
                let i_2: usize = program[i + 2] as usize;
                let store_index: usize = program[i + 3] as usize;

                let value_1 = program[i_1];
                let value_2 = program[i_2];
                program[store_index] = value_1 + value_2;
                offset = 4;
            }
            2 => {
                // multiply - opcode, v1, v2, store_index
                let i_1: usize = program[i + 1] as usize;
                let i_2: usize = program[i + 2] as usize;
                let store_index: usize = program[i + 3] as usize;

                let value_1 = program[i_1];
                let value_2 = program[i_2];
                program[store_index] = value_1 * value_2;
                offset = 4;
            }
            3 => {
                // save
                // Opcode 3 takes a single integer as input and saves it to the
                // position given by its only parameter. For example, the
                // instruction 3,50 would take an input value and store it at
                // address 50.
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        match input.trim().parse::<i32>() {
                            Ok(value) => {
                                let i_1: usize = program[i + 1] as usize;
                                program[i_1] = value;
                            }
                            Err(error) => {
                                println!("Error: {}", error);
                            }
                        }
                    }
                    Err(error) => {
                        println!("Error: {}", error);
                    }
                }
                offset = 2;
            }
            4 => {
                // output
                // Opcode 4 outputs the value of its only parameter. For
                // example, the instruction 4,50 would output the value at
                // address 50.
                let i_1: usize = program[i + 1] as usize;
                println!("Output: {}", program[i_1]);
                offset = 2;
            }
            99 => {
                break;
            }
            _ => panic!("Error"),
        }
        i += offset;
    }

    program
}
