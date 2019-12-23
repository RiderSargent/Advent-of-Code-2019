use std::fs;
use std::io;
use std::io::prelude::*;

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
        let mut full_opcode: Vec<i32> = digitize(&program[i]);
        let opcode: i32;
        let mut parameter_modes: Vec<i32>;
        let offset: usize;

        // println!("program[i]: {:?}", program[i]);

        // set opcode and parameter modes
        if program[i] < 100 {
            opcode = program[i];
            parameter_modes = vec![];
        } else {
            opcode = full_opcode.pop().unwrap() + full_opcode.pop().unwrap() * 10;
            parameter_modes = full_opcode;
        }

        // println!("parameter_modes: {:?}", parameter_modes);

        match opcode {
            01 => {
                // ADD - opcode, read 1 index, read 2 index, write index
                let value_1: i32;
                let value_2: i32;

                match parameter_modes.pop() {
                    None | Some(0) => {
                        let i_1: usize = program[i + 1] as usize;
                        value_1 = program[i_1];
                    }
                    Some(1) => {
                        value_1 = program[i + 1];
                    }
                    _ => panic!("Error")
                }

                match parameter_modes.pop() {
                    None | Some(0) => {
                        let i_2: usize = program[i + 2] as usize;
                        value_2 = program[i_2];
                    }
                    Some(1) => {
                        value_2 = program[i + 2];
                    }
                    _ => panic!("Error")
                }

                let store_index: usize = program[i + 3] as usize;

                program[store_index] = value_1 + value_2;
                offset = 4;
            }
            02 => {
                // MULTIPLY - opcode, read 1 index, read 2 index, write index
                let value_1;
                let value_2;

                match parameter_modes.pop() {
                    None | Some(0) => {
                        let i_1: usize = program[i + 1] as usize;
                        value_1 = program[i_1];
                    }
                    Some(1) => {
                        value_1 = program[i + 1];
                    }
                    _ => panic!("Error")
                }

                match parameter_modes.pop() {
                    None | Some(0) => {
                        let i_2: usize = program[i + 2] as usize;
                        value_2 = program[i_2];
                    }
                    Some(1) => {
                        value_2 = program[i + 2];
                    }
                    _ => panic!("Error")
                }

                let store_index: usize = program[i + 3] as usize;

                program[store_index] = value_1 * value_2;
                offset = 4;
            }
            03 => {
                // INPUT - opcode, write index
                // Opcode 3 takes a single integer as input and saves it to the
                // position given by its only parameter. For example, the
                // instruction 3,50 would take an input value and store it at
                // address 50.
                let i_1: usize = program[i + 1] as usize;
                let mut input = String::new();

                print!("Input: ");
                io::stdout().flush().unwrap();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        match input.trim().parse::<i32>() {
                            Ok(value) => {
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
            04 => {
                // OUTPUT - opcode, read index
                // Opcode 4 outputs the value of its only parameter. For
                // example, the instruction 4,50 would output the value at
                // address 50.
                let value;

                match parameter_modes.pop() {
                    None | Some(0) => {
                        let i_1: usize = program[i + 1] as usize;
                        value = program[i_1];
                    }
                    Some(1) => {
                        value = program[i + 1];
                    }
                    _ => panic!("Error")
                }

                println!("Output: {}", value);
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

fn digitize(n: &i32) -> Vec<i32> {
    let mut digits: Vec<i32> = Vec::new();
    let mut remaining = n;
    let mut next;

    while remaining > &9 {
        digits.push(remaining % 10);
        next = remaining / 10;
        remaining = &next;
    }
    digits.push(*remaining);
    digits.reverse();

    digits
}
