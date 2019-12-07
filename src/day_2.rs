use std::fs;

pub fn exercise_1() {
    let mut program: Vec<usize> = get_program();

    // "before running the program, replace position 1 with the value 12
    // and replace position 2 with the value 2."
    program[1] = 12;
    program[2] = 2;

    let final_program = run_program(program);

    println!("[D2E1] Value at position 0: {:?}", final_program[0]);
}

pub fn exercise_2() {
    let mut current_program: Vec<usize>;
    let mut final_program: Vec<usize> = vec![];

    'outer: for i in 0..100 {
        'inner: for j in 0..100 {
            current_program = get_program();
            current_program[1] = i;
            current_program[2] = j;

            final_program = run_program(current_program);
            if final_program[0] == 19690720 {
                break 'outer;
            }
        }
    }

    println!(
        "[D2E2] 100 * noun + verb: {:?}{:?}",
        final_program[1], final_program[2]
    );
}

fn get_program() -> Vec<usize> {
    let filename: &str = "input_day_02.txt";
    let raw_program: String = fs::read_to_string(filename).expect("Error reading file");
    let program: Vec<usize> = raw_program
        .trim()
        .split(',')
        .into_iter()
        .map(|n| n.parse::<usize>())
        .filter_map(Result::ok)
        .collect();

    program
}

fn run_program(mut program: Vec<usize>) -> Vec<usize> {
    let mut i = 0;
    let mut opcode;
    let mut value_1;
    let mut value_2;
    let mut store_index;
    let program_length = program.len();

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

    program
}
