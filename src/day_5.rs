use crate::intcode;

pub fn exercise_1() {
    // let program = intcode::load_program("input_day_05.txt");
    let program: Vec<i32> = vec![3, 0, 4, 0, 99];

    println!("{:?}", program);

    let final_program = intcode::run_program(program);

    println!("[D5E1]: {:?}", final_program);
}
