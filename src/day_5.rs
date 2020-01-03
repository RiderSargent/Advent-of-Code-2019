use crate::intcode;

pub fn exercise_1() -> Vec<i32> {
    let program = intcode::load_program("input_day_05.txt");

    intcode::run_program_output(vec![1], program)
}

pub fn exercise_2() -> Vec<i32> {
    let program = intcode::load_program("input_day_05.txt");

    intcode::run_program_output(vec![5], program)
}
