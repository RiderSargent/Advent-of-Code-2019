use crate::intcode;
use itertools::Itertools;

pub fn exercise_1() -> i32 {
    let permutations = (0..5).permutations(5);
    let mut max = 0;

    for p in permutations {
        let output = run_amps(&p);
        if output > max {
            max = output;
        }
    }

    max
}

fn run_amps(phase_settings: &Vec<i32>) -> i32 {
    // should be 43210
    // run_amps(vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0], vec![4,3,2,1,0])

    // should be 54321
    // run_amps(vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0], vec![0,1,2,3,4])

    // should be 65210
    // run_amps(vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0], vec![1,0,4,3,2])

    // let amp = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];

    let amp = intcode::load_program("input_day_07.txt");
    let phase_settings: Vec<i32> = phase_settings.clone();

    let mut signal: i32 = 0;

    for s in phase_settings {
        signal = intcode::run_program_output(
            vec![signal, s],
            amp.clone()
        );
    }

    signal
}
