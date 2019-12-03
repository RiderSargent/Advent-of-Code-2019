use std::fs;

#[allow(dead_code)]
pub fn exercise_1() {
    let filename: &str = "input_day_02.txt";
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
        value_1 = program[program[i + 1]];
        value_2 = program[program[i + 2]];
        store_index = program[i + 3];

        match opcode {
            1 => {
                program[store_index] = value_1 + value_2;
            }
            2 => {
                program[store_index] = value_1 * value_2;
            }
            99 => {
                break;
            }
            _ => panic!("Error"),
        }
        i += 4;
    }

    println!("[D2E1] value at position 0: {:?}", program[0]);
}
