use crate::intcode;

pub fn exercise_1() {
    let program = intcode::load_program("input_day_05.txt");

    println!("\n[D5E1]:");
    intcode::run_program_interactive(vec![1], program);
}

pub fn exercise_2() {
    let program = intcode::load_program("input_day_05.txt");

    // --- Test programs ---
    // For example, here are several programs that take one input,
    // compare it to the value 8, and then produce one output:

    // output 1 if input is equal to 8, else 0 (position mode)
    // let program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];

    // output 1 if input is less than 8 else 0 (position mode)
    // let program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];

    // output 1 if input is equal to 8, else 0 (immediate mode)
    // let program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];

    // output 1 if input is less than 8, else 0 (immediate mode)
    // let program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];

    // Here are some jump tests that take an input, then output 0 if the
    // input was zero or 1 if the input was non-zero:

    // ...(using position mode):
    // let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];

    // ...(using immediate mode)
    // let program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

    // This program uses an input instruction to ask for a single number.
    // output 999 if the input value is below 8,
    // output 1000 if the input value is equal to 8, or
    // output 1001 if the input value is greater than 8.
    // let program = vec![3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99];

    println!("\n[D5E2]:");
    intcode::run_program_interactive(vec![5], program);
}
