use crate::intcode;

pub fn exercise_1() {
    let mut program: Vec<i32> = intcode::load_program("input_day_02.txt");

    // "before running the program, replace position 1 with the value 12
    // and replace position 2 with the value 2."
    program[1] = 12;
    program[2] = 2;

    let final_program = intcode::run_program(program);

    // should be: 3166704
    println!("[D2E1] Value at position 0: {:?}", final_program[0]);
}

pub fn exercise_2() {
    let mut current_program: Vec<i32>;
    let mut final_program: Vec<i32> = vec![];

    'outer: for i in 0..100 {
        'inner: for j in 0..100 {
            current_program = intcode::load_program("input_day_02.txt");
            current_program[1] = i;
            current_program[2] = j;

            final_program = intcode::run_program(current_program);
            if final_program[0] == 19690720 {
                break 'outer;
            }
        }
    }

    // should be 8018
    println!(
        "[D2E2] 100 * noun + verb: {:?}{:?}",
        final_program[1], final_program[2]
    );
}
