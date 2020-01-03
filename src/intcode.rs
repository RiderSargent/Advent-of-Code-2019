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

pub fn run_program_program(input: Vec<i32>, program: Vec<i32>) -> Vec<i32> {
    run_program(input, program).0
}

pub fn run_program_output(input: Vec<i32>, program: Vec<i32>) -> Vec<i32> {
    run_program(input, program).1
}

fn run_program(mut input: Vec<i32>, mut program: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    // 01 - ADD (opcode, p1, p2, write_index)
    // 02 - MULTIPLY (opcode, p1, p2, write_index)
    // 03 - INPUT (opcode, write_index)
    // 04 - OUTPUT (opcode, read_index)
    // 05 - JUMP-IF-TRUE (opcode, condition, new_pointer_target)
    // 06 - JUMP-IF-FALSE (opcode, condition, new_pointer_target)
    // 07 - LESS-THAN (opcode, first_param, second_param, write_index)
    // 08 - EQUALS (opcode, first_param, second_param, write_index)
    // 99 - TERMINATE

    let mut output: Vec<i32> = vec![];
    let mut i = 0;

    while i < program.len() && program[i] != 99 {
        let opcode: Intcode = get_opcode(program[i] % 100);

        let mode_1: i32 = (program[i] / 100) % 10;
        let mode_2: i32 = (program[i] / 1000) % 10;

        match opcode {
            Intcode::Add => {
                // ADD - opcode, read 1 index, read 2 index, write index
                let value_1 = get_value(&program, i, mode_1);
                let value_2 = get_value(&program, i + 1, mode_2);

                set_value(&mut program, i + 3, value_1 + value_2);

                i += 4;
            }

            Intcode::Multiply => {
                // MULTIPLY - opcode, read 1 index, read 2 index, write index
                let value_1 = get_value(&program, i, mode_1);
                let value_2 = get_value(&program, i + 1, mode_2);

                set_value(&mut program, i + 3, value_1 * value_2);
                i += 4;
            }

            Intcode::Input => {
                // INPUT - opcode, write index
                // Opcode 3 takes a single integer as input and saves it to the
                // position given by its only parameter. For example, the
                // instruction 3,50 would take an input value and store it at
                // address 50.
                let i_1: usize = program[i + 1] as usize;

                match input.pop() {
                    Some(value) => {
                        // TODO: Why can't I do: set_value(&mut program, i_1, value);
                        program[i_1] = value;
                    },
                    None => { println!("Error: Missing input."); },
                }
                i += 2;
            }

            Intcode::Output => {
                // OUTPUT - opcode, read index
                // Opcode 4 outputs the value of its only parameter. For
                // example, the instruction 4,50 would output the value at
                // address 50.
                let value_1 = get_value(&program, i, mode_1);

                output.push(value_1);
                i += 2;
            }

            Intcode::JumpIfTrue => {
                // Jump-if-true: if the first parameter is non-zero, it
                // sets the instruction pointer to the value from the second
                // parameter. Otherwise, it does nothing.
                let value_1 = get_value(&program, i, mode_1);
                let value_2 = get_value(&program, i + 1, mode_2);

                if value_1 != 0 {
                    i = value_2 as usize;
                } else {
                    i += 3;
                }
            }

            Intcode::JumpIfFalse => {
                // Jump-if-false: if the first parameter is zero, it
                // sets the instruction pointer to the value from the second
                // parameter. Otherwise, it does nothing.
                let value_1 = get_value(&program, i, mode_1);
                let value_2 = get_value(&program, i + 1, mode_2);

                if value_1 == 0 {
                    i = value_2 as usize;
                } else {
                    i += 3;
                }
            }

            Intcode::LessThan => {
                // Less than: if the first parameter is less than the
                // second parameter, it stores 1 in the position given by the
                // third parameter. Otherwise, it stores 0.
                let value_1 = get_value(&program, i, mode_1);
                let value_2 = get_value(&program, i + 1, mode_2);

                if value_1 < value_2 {
                    set_value(&mut program, i + 3, 1);
                } else {
                    set_value(&mut program, i + 3, 0);
                }
                i += 4;
            }

            Intcode::Equals => {
                // Equals: if the first parameter is equal to the second
                // parameter, it stores 1 in the position given by the third
                // parameter. Otherwise, it stores 0.
                let value_1 = get_value(&program, i, mode_1);
                let value_2 = get_value(&program, i + 1, mode_2);

                if value_1 == value_2 {
                    set_value(&mut program, i + 3, 1);
                } else {
                    set_value(&mut program, i + 3, 0);
                }
                i += 4;
            }

            Intcode::Terminate => {
                break;
            }
        }
    }

    (program, output)
}

enum Intcode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Terminate,
}

fn get_opcode(code: i32) -> Intcode {
    match code {
        1 => Intcode::Add,
        2 => Intcode::Multiply,
        3 => Intcode::Input,
        4 => Intcode::Output,
        5 => Intcode::JumpIfTrue,
        6 => Intcode::JumpIfFalse,
        7 => Intcode::LessThan,
        8 => Intcode::Equals,
        99 => Intcode::Terminate,
        _ => panic!("Error: Invalid Intcode"),
    }
}

fn get_value(program: &Vec<i32>, index: usize, mode: i32) -> i32 {
    let value: i32 = match mode {
        0 => program[program[index + 1] as usize],
        _ => program[index + 1],
    };

    value
}

fn set_value(program: &mut Vec<i32>, index: usize, value: i32) {
    let write_index: usize = program[index] as usize;
    program[write_index] = value;
}
