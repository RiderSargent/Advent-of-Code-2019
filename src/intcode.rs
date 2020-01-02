use std::fs;

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

pub fn run_program(mut input: Vec<i32>, mut program: Vec<i32>) -> Vec<i32> {
    // 01 - ADD (opcode, p1, p2, write_index)
    // 02 - MULTIPLY (opcode, p1, p2, write_index)
    // 03 - INPUT (opcode, write_index)
    // 04 - OUTPUT (opcode, read_index)
    // 05 - JUMP-IF-TRUE (opcode, condition, new_pointer_target)
    // 06 - JUMP-IF-FALSE (opcode, condition, new_pointer_target)
    // 07 - LESS-THAN (opcode, first_param, second_param, write_index)
    // 08 - EQUALS (opcode, first_param, second_param, write_index)
    // 99 - TERMINATE

    let mut i = 0;

    while i < program.len() && program[i] != 99 {
        let mut full_opcode: Vec<i32> = digitize(&program[i]);
        let opcode: i32;
        let mut parameter_modes: Vec<i32>;

        // set opcode and parameter modes
        if program[i] < 100 {
            opcode = program[i];
            parameter_modes = vec![];
        } else {
            opcode = full_opcode.pop().unwrap() + full_opcode.pop().unwrap() * 10;
            parameter_modes = full_opcode;
        }

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
                    Some(_) => {
                        value_1 = program[i + 1];
                    }
                }

                match parameter_modes.pop() {
                    None | Some(0) => {
                        let i_2: usize = program[i + 2] as usize;
                        value_2 = program[i_2];
                    }
                    Some(_) => {
                        value_2 = program[i + 2];
                    }
                }

                let store_index: usize = program[i + 3] as usize;

                program[store_index] = value_1 + value_2;
                i += 4;
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
                    Some(_) => {
                        value_1 = program[i + 1];
                    }
                }

                match parameter_modes.pop() {
                    None | Some(0) => {
                        let i_2: usize = program[i + 2] as usize;
                        value_2 = program[i_2];
                    }
                    Some(_) => {
                        value_2 = program[i + 2];
                    }
                }

                let store_index: usize = program[i + 3] as usize;

                program[store_index] = value_1 * value_2;
                i += 4;
            }

            03 => {
                // INPUT - opcode, write index
                // Opcode 3 takes a single integer as input and saves it to the
                // position given by its only parameter. For example, the
                // instruction 3,50 would take an input value and store it at
                // address 50.
                let i_1: usize = program[i + 1] as usize;

                match input.pop() {
                    Some(foo) => match foo.to_string().parse::<i32>() {
                        Ok(value) => {
                            program[i_1] = value;
                        }
                        Err(error) => {
                            println!("Error: {}", error);
                        }
                    },
                    None => {
                        println!("Error: Missing input.");
                    }
                }
                i += 2;
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
                    Some(_) => {
                        value = program[i + 1];
                    }
                }

                println!("  Output: {}", value);
                i += 2;
            }

            05 => {
                // Jump-if-true: if the first parameter is non-zero, it
                // sets the instruction pointer to the value from the second
                // parameter. Otherwise, it does nothing.
                let value_1: i32;
                let value_2: i32;

                match parameter_modes.pop() {
                    None | Some(0) => {
                        let i_1: usize = program[i + 1] as usize;
                        value_1 = program[i_1];
                    }
                    Some(_) => {
                        value_1 = program[i + 1];
                    }
                }

                match parameter_modes.pop() {
                    None | Some(0) => {
                        let i_2: usize = program[i + 2] as usize;
                        value_2 = program[i_2];
                    }
                    Some(_) => {
                        value_2 = program[i + 2];
                    }
                }

                if value_1 != 0 {
                    i = value_2 as usize;
                } else {
                    i += 3;
                }
            }

            06 => {
                // Jump-if-false: if the first parameter is zero, it
                // sets the instruction pointer to the value from the second
                // parameter. Otherwise, it does nothing.
                let value_1: i32;
                let value_2: i32;

                match parameter_modes.pop() {
                    None | Some(0) => {
                        let i_1: usize = program[i + 1] as usize;
                        value_1 = program[i_1];
                    }
                    Some(_) => {
                        value_1 = program[i + 1];
                    }
                }

                match parameter_modes.pop() {
                    None | Some(0) => {
                        let i_2: usize = program[i + 2] as usize;
                        value_2 = program[i_2];
                    }
                    Some(_) => {
                        value_2 = program[i + 2];
                    }
                }

                if value_1 == 0 {
                    i = value_2 as usize;
                } else {
                    i += 3;
                }
            }

            07 => {
                // Less than: if the first parameter is less than the
                // second parameter, it stores 1 in the position given by the
                // third parameter. Otherwise, it stores 0.
                let value_1: i32;
                let value_2: i32;

                match parameter_modes.pop() {
                    None | Some(0) => {
                        let i_1: usize = program[i + 1] as usize;
                        value_1 = program[i_1];
                    }
                    Some(_) => {
                        value_1 = program[i + 1];
                    }
                }

                match parameter_modes.pop() {
                    None | Some(0) => {
                        let i_2: usize = program[i + 2] as usize;
                        value_2 = program[i_2];
                    }
                    Some(_) => {
                        value_2 = program[i + 2];
                    }
                }

                let store_index: usize = program[i + 3] as usize;

                if value_1 < value_2 {
                    program[store_index] = 1;
                } else {
                    program[store_index] = 0;
                }
                i += 4;
            }

            08 => {
                // Equals: if the first parameter is equal to the second
                // parameter, it stores 1 in the position given by the third
                // parameter. Otherwise, it stores 0.
                let value_1: i32;
                let value_2: i32;

                match parameter_modes.pop() {
                    None | Some(0) => {
                        let i_1: usize = program[i + 1] as usize;
                        value_1 = program[i_1];
                    }
                    Some(_) => {
                        value_1 = program[i + 1];
                    }
                }

                match parameter_modes.pop() {
                    None | Some(0) => {
                        let i_2: usize = program[i + 2] as usize;
                        value_2 = program[i_2];
                    }
                    Some(_) => {
                        value_2 = program[i + 2];
                    }
                }

                let store_index: usize = program[i + 3] as usize;

                if value_1 == value_2 {
                    program[store_index] = 1;
                } else {
                    program[store_index] = 0;
                }
                i += 4;
            }

            99 => {
                break;
            }

            _ => panic!("Error"),
        }
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
