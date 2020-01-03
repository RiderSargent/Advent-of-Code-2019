#[cfg(test)]
use super::*;

// --- Intcode -----------------------------------------------------------------
#[test]
fn test_day_2_intcode_1() {
    let input: Vec<i32> = vec![];
    let program: Vec<i32> = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    let expected: Vec<i32> = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
    assert_eq!(expected, intcode::run_program_program(input, program));
}

#[test]
fn test_day_2_intcode_2() {
    let input: Vec<i32> = vec![];
    let program: Vec<i32> = vec![1, 0, 0, 0, 99];
    let expected: Vec<i32> = vec![2, 0, 0, 0, 99];
    assert_eq!(expected, intcode::run_program_program(input, program));
}

#[test]
fn test_day_2_intcode_3() {
    let input: Vec<i32> = vec![];
    let program: Vec<i32> = vec![2, 3, 0, 3, 99];
    let expected: Vec<i32> = vec![2, 3, 0, 6, 99];
    assert_eq!(expected, intcode::run_program_program(input, program));
}

#[test]
fn test_day_2_intcode_4() {
    let input: Vec<i32> = vec![];
    let program: Vec<i32> = vec![2, 4, 4, 5, 99, 0];
    let expected: Vec<i32> = vec![2, 4, 4, 5, 99, 9801];
    assert_eq!(expected, intcode::run_program_program(input, program));
}

#[test]
fn test_day_2_intcode_5() {
    let input: Vec<i32> = vec![];
    let program: Vec<i32> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    let expected: Vec<i32> = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
    assert_eq!(expected, intcode::run_program_program(input, program));
}

#[test]
fn test_day_5_intcode_1() {
    let input: Vec<i32> = vec![17];
    let program: Vec<i32> = vec![3,0,4,0,99];
    let expected: Vec<i32> = vec![17];
    assert_eq!(expected, intcode::run_program_output(input, program));
}

#[test]
fn test_day_5_intcode_2() {
    let input: Vec<i32> = vec![];
    let program: Vec<i32> = vec![1002, 4, 3, 4, 33];
    let expected: Vec<i32> = vec![1002, 4, 3, 4, 99];
    assert_eq!(expected, intcode::run_program_program(input, program));
}

#[test]
fn test_day_5_intcode_3() {
    let input: Vec<i32> = vec![];
    let program: Vec<i32> = vec![1101, 100, -1, 4, 0];
    let expected: Vec<i32> = vec![1101, 100, -1, 4, 99];
    assert_eq!(expected, intcode::run_program_program(input, program));
}

#[test]
fn test_day_5_intcode_4() {
    // output 1 if input == 8, otherwise 0 (using position mode)
    let program: Vec<i32> = vec![3,9,8,9,10,9,4,9,99,-1,8];
    assert_eq!(vec![1], intcode::run_program_output(vec![8], program.clone()));
    assert_eq!(vec![0], intcode::run_program_output(vec![3], program.clone()));
}

#[test]
fn test_day_5_intcode_5() {
    // output 1 if input == 8, otherwise 0 (using immediate mode)
    let program: Vec<i32> = vec![3,3,1108,-1,8,3,4,3,99];
    assert_eq!(vec![1], intcode::run_program_output(vec![8], program.clone()));
    assert_eq!(vec![0], intcode::run_program_output(vec![3], program.clone()));
}

#[test]
fn test_day_5_intcode_6() {
    // output 1 if input < 8, otherwise 0 (using position mode)
    let program: Vec<i32> = vec![3,9,7,9,10,9,4,9,99,-1,8];
    assert_eq!(vec![1], intcode::run_program_output(vec![7], program.clone()));
    assert_eq!(vec![0], intcode::run_program_output(vec![9], program.clone()));
}

#[test]
fn test_day_5_intcode_7() {
    // output 1 if input < 8, otherwise 0 (using immediate mode)
    let program: Vec<i32> = vec![3,3,1107,-1,8,3,4,3,99];
    assert_eq!(vec![1], intcode::run_program_output(vec![7], program.clone()));
    assert_eq!(vec![0], intcode::run_program_output(vec![9], program.clone()));
}

#[test]
fn test_day_5_intcode_8() {
    // output 0 if input == 0, otherwise 1 (using position mode)
    let program: Vec<i32> = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
    assert_eq!(vec![1], intcode::run_program_output(vec![7], program.clone()));
    assert_eq!(vec![0], intcode::run_program_output(vec![0], program.clone()));
}

#[test]
fn test_day_5_intcode_9() {
    // output 0 if input == 0, otherwise 1 (using immediate mode)
    let program: Vec<i32> = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
    assert_eq!(vec![1], intcode::run_program_output(vec![7], program.clone()));
    assert_eq!(vec![0], intcode::run_program_output(vec![0], program.clone()));
}

#[test]
fn test_day_5_intcode_10() {
    // output 999 if input < 8; 1000 if input == 8; 1001 if input > 8
    let program: Vec<i32> = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
    assert_eq!(vec![999], intcode::run_program_output(vec![7], program.clone()));
    assert_eq!(vec![1000], intcode::run_program_output(vec![8], program.clone()));
    assert_eq!(vec![1001], intcode::run_program_output(vec![10], program.clone()));
}


// --- Acceptance Tests --------------------------------------------------------
#[test]
fn test_d1e1() {
    assert_eq!(3303995, day_1::exercise_1());
}

#[test]
fn test_d1e2() {
    assert_eq!(4953118, day_1::exercise_2());
}

#[test]
fn test_d2e1() {
    assert_eq!(3166704, day_2::exercise_1());
}

#[test]
fn test_d2e2() {
    assert_eq!(8018, day_2::exercise_2());
}

#[test]
fn test_d3e1() {
    assert_eq!(248, day_3::exercise_1());
}

#[test]
fn test_d3e2() {
    assert_eq!(28580, day_3::exercise_2());
}

#[test]
fn test_d4e1() {
    assert_eq!(1955, day_4::exercise_1());
}

#[test]
fn test_d4e2() {
    assert_eq!(1319, day_4::exercise_2());
}

#[test]
fn test_d5e1() {
    let expected = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 8332629];
    assert_eq!(expected, day_5::exercise_1());
}

#[test]
fn test_d5e2() {
    assert_eq!(vec![8805067], day_5::exercise_2());
}

#[test]
fn test_d6e1() {
    assert_eq!(294191, day_6::exercise_1());
}

#[test]
fn test_d6e2() {
    assert_eq!(424, day_6::exercise_2());
}

#[test]
fn test_d7e1() {
    assert_eq!(24625, day_7::exercise_1());
}
