use crate::intcode;

pub fn exercise_1() -> Vec<i32> {
    let program = intcode::load_program("input_day_05.txt");

    intcode::run_program(vec![1], program).1
}

pub fn exercise_2() -> Vec<i32> {
    let program = intcode::load_program("input_day_05.txt");

    intcode::run_program(vec![5], program).1
}
