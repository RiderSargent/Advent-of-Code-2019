use crate::intcode;

pub fn exercise_1() {
    let program = intcode::load_program("input_day_05.txt");

    // should be 8332629 (preceeded by 9 output 0s)
    println!("[D5E1]");
    intcode::run_program_interactive(vec![1], program);
}

pub fn exercise_2() {
    let program = intcode::load_program("input_day_05.txt");

    // should be 8805067
    println!("[D5E2]");
    intcode::run_program_interactive(vec![5], program);
}
