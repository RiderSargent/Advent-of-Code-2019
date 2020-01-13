#[cfg(test)]
use super::*;

// --- Intcode -----------------------------------------------------------------
#[test]
fn test_day_2_intcode_1() {
    let input: Option<i32> = None;
    let mut program: intcode::Program = intcode::Program::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
    let expected: Vec<i32> = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

    program.run(input, 0);

    assert_eq!(expected, program.memory);
}

#[test]
fn test_day_2_intcode_2() {
    let input: Option<i32> = None;
    let mut program: intcode::Program = intcode::Program::new(vec![1, 0, 0, 0, 99]);
    let expected: Vec<i32> = vec![2, 0, 0, 0, 99];

    program.run(input, 0);

    assert_eq!(expected, program.memory);
}

#[test]
fn test_day_2_intcode_3() {
    let input: Option<i32> = None;
    let mut program: intcode::Program = intcode::Program::new(vec![2, 3, 0, 3, 99]);
    let expected: Vec<i32> = vec![2, 3, 0, 6, 99];

    program.run(input, 0);

    assert_eq!(expected, program.memory);
}

#[test]
fn test_day_2_intcode_4() {
    let input: Option<i32> = None;
    let mut program: intcode::Program = intcode::Program::new(vec![2, 4, 4, 5, 99, 0]);
    let expected: Vec<i32> = vec![2, 4, 4, 5, 99, 9801];

    program.run(input, 0);

    assert_eq!(expected, program.memory);
}

#[test]
fn test_day_2_intcode_5() {
    let input: Option<i32> = None;
    let mut program: intcode::Program = intcode::Program::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
    let expected: Vec<i32> = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];

    program.run(input, 0);

    assert_eq!(expected, program.memory);
}

#[test]
fn test_day_5_intcode_1() {
    let input: Option<i32> = Some(17);
    let mut program: intcode::Program = intcode::Program::new(vec![3,0,4,0,99]);

    assert_eq!(17, program.run(input, 0).unwrap());
}

#[test]
fn test_day_5_intcode_2() {
    let input: Option<i32> = None;
    let mut program: intcode::Program = intcode::Program::new(vec![1002, 4, 3, 4, 33]);
    let expected: Vec<i32> = vec![1002, 4, 3, 4, 99];

    program.run(input, 0);

    assert_eq!(expected, program.memory);
}

#[test]
fn test_day_5_intcode_3() {
    let input: Option<i32> = None;
    let mut program: intcode::Program = intcode::Program::new(vec![1101, 100, -1, 4, 0]);
    let expected: Vec<i32> = vec![1101, 100, -1, 4, 99];

    program.run(input, 0);

    assert_eq!(expected, program.memory);
}

#[test]
fn test_day_5_intcode_4_equal() {
    // output 1 if input == 8, otherwise 0 (using position mode)
    let mut program: intcode::Program = intcode::Program::new(vec![3,9,8,9,10,9,4,9,99,-1,8]);

    assert_eq!(1, program.run(Some(8), 0).unwrap());
}

#[test]
fn test_day_5_intcode_4_not_equal() {
    // output 1 if input == 8, otherwise 0 (using position mode)
    let mut program: intcode::Program = intcode::Program::new(vec![3,9,8,9,10,9,4,9,99,-1,8]);

    assert_eq!(0, program.run(Some(3), 0).unwrap());
}

#[test]
fn test_day_5_intcode_5_equal() {
    // output 1 if input == 8, otherwise 0 (using immediate mode)
    let mut program: intcode::Program = intcode::Program::new(vec![3,3,1108,-1,8,3,4,3,99]);

    assert_eq!(1, program.run(Some(8), 0).unwrap());
}

#[test]
fn test_day_5_intcode_5_not_equal() {
    // output 1 if input == 8, otherwise 0 (using immediate mode)
    let mut program: intcode::Program = intcode::Program::new(vec![3,3,1108,-1,8,3,4,3,99]);

    assert_eq!(0, program.run(Some(3), 0).unwrap());
}

#[test]
fn test_day_5_intcode_6_less_than() {
    // output 1 if input < 8, otherwise 0 (using position mode)
    let mut program: intcode::Program = intcode::Program::new(vec![3,9,7,9,10,9,4,9,99,-1,8]);

    assert_eq!(1, program.run(Some(7), 0).unwrap());
}

#[test]
fn test_day_5_intcode_6_greater_than() {
    // output 1 if input < 8, otherwise 0 (using position mode)
    let mut program: intcode::Program = intcode::Program::new(vec![3,9,7,9,10,9,4,9,99,-1,8]);

    assert_eq!(0, program.run(Some(9), 0).unwrap());
}

#[test]
fn test_day_5_intcode_7_less_than() {
    // output 1 if input < 8, otherwise 0 (using immediate mode)
    let mut program: intcode::Program = intcode::Program::new(vec![3,3,1107,-1,8,3,4,3,99]);

    assert_eq!(1, program.run(Some(7), 0).unwrap());
}

#[test]
fn test_day_5_intcode_7_greater_than() {
    // output 1 if input < 8, otherwise 0 (using immediate mode)
    let mut program: intcode::Program = intcode::Program::new(vec![3,3,1107,-1,8,3,4,3,99]);

    assert_eq!(0, program.run(Some(9), 0).unwrap());
}

#[test]
fn test_day_5_intcode_8_equal() {
    // output 0 if input == 0, otherwise 1 (using position mode)
    let mut program: intcode::Program = intcode::Program::new(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9]);

    assert_eq!(0, program.run(Some(0), 0).unwrap());
}

#[test]
fn test_day_5_intcode_8_not_equal() {
    // output 0 if input == 0, otherwise 1 (using position mode)
    let mut program: intcode::Program = intcode::Program::new(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9]);

    assert_eq!(1, program.run(Some(7), 0).unwrap());
}

#[test]
fn test_day_5_intcode_9_equal() {
    // output 0 if input == 0, otherwise 1 (using immediate mode)
    let mut program: intcode::Program = intcode::Program::new(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1]);

    assert_eq!(1, program.run(Some(7), 0).unwrap());
}

#[test]
fn test_day_5_intcode_9_not_equal() {
    // output 0 if input == 0, otherwise 1 (using immediate mode)
    let mut program: intcode::Program = intcode::Program::new(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1]);

    assert_eq!(0, program.run(Some(0), 0).unwrap());
}

#[test]
fn test_day_5_intcode_10_less_than() {
    // output 999 if input < 8; 1000 if input == 8; 1001 if input > 8
    let mut program: intcode::Program = intcode::Program::new(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99]);

    assert_eq!(999, program.run(Some(7), 0).unwrap());
}

#[test]
fn test_day_5_intcode_10_equal() {
    // output 999 if input < 8; 1000 if input == 8; 1001 if input > 8
    let mut program: day_5::Program = day_5::Program::new(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99]);

    assert_eq!(1000, program.run(Some(8), 0).last().unwrap().unwrap());
}

#[test]
fn test_day_5_intcode_10_greater_than() {
    // output 999 if input < 8; 1000 if input == 8; 1001 if input > 8
    let mut program: day_5::Program = day_5::Program::new(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99]);

    assert_eq!(1001, program.run(Some(10), 0).last().unwrap().unwrap());
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
    assert_eq!(8332629, day_5::exercise_1());
}

#[test]
fn test_d5e2() {
    assert_eq!(8805067, day_5::exercise_2());
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
