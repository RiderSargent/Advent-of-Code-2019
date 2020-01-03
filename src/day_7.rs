use crate::intcode;
use itertools::Itertools;

pub fn exercise_1() -> i32 {
    let program = intcode::load_program("input_day_07.txt");

    // should be 43210
    // run_amps(vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0], vec![4,3,2,1,0])

    // should be 54321
    // run_amps(vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0], vec![0,1,2,3,4])

    // should be 65210
    // run_amps(vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0], vec![1,0,4,3,2])

    // let program = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];

    let permutations = (0..5).permutations(5);
    // println!("permutations: {:?}", permutations);

    // let mut best_signal_order: Vec<i32> = vec![];
    let mut max = 0;

    for p in permutations {
        let output = run_amps(program.clone(), &p);
        if output > max {
            max = output;
            // best_signal_order = p.clone();
        }
    }

    // println!("best_signal_order: {:?}", best_signal_order);

    max
}

fn run_amps(program: Vec<i32>, signals: &Vec<i32>) -> i32 {
    let mut s: Vec<i32> = signals.clone();

    s.reverse();

    let a_output = intcode::run_program_output(
        vec![0, s.pop().unwrap()],
        program.clone()
    );
    let b_output = intcode::run_program_output(
        vec![*a_output.last().unwrap(), s.pop().unwrap()],
        program.clone(),
    );
    let c_output = intcode::run_program_output(
        vec![*b_output.last().unwrap(), s.pop().unwrap()],
        program.clone(),
    );
    let d_output = intcode::run_program_output(
        vec![*c_output.last().unwrap(), s.pop().unwrap()],
        program.clone(),
    );
    let e_output = intcode::run_program_output(
        vec![*d_output.last().unwrap(), s.pop().unwrap()],
        program.clone(),
    );

    *e_output.last().unwrap()
}
