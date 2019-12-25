use crate::intcode;

pub fn exercise_1() {
    let program = intcode::load_program("input_day_05.txt");

    intcode::run_program_interactive(vec![1], program);
}
