use std::fs;

pub fn exercise_1() -> i32 {
    let mut program: Program = Program::new_from_file("input_day_05.txt");

    program.run(Some(1), 0).last().unwrap().unwrap()
}

pub fn exercise_2() -> i32 {
    let mut program: Program = Program::new_from_file("input_day_05.txt");

    program.run(Some(5), 0).last().unwrap().unwrap()
}

fn get(program: &Vec<i32>, index: usize, mode: i32) -> i32 {
    let value: i32 = match mode {
        0 => program[program[index + 1] as usize],
        _ => program[index + 1],
    };

    value
}

fn set(program: &mut Vec<i32>, index: usize, value: i32) {
    let write_index: usize = program[index] as usize;

    program[write_index] = value;
}

enum Intcode {
    Add(i32, i32),
    Multiply(i32, i32),
    Input(i32),
    Output(i32),
    JumpIfTrue(i32, i32),
    JumpIfFalse(i32, i32),
    LessThan(i32, i32),
    Equals(i32, i32),
    Terminate,
}

impl Intcode {
    fn from(n: i32) -> Intcode {
        let pm_1: i32 = (n / 100) % 10;
        let pm_2: i32 = (n / 1000) % 10;

        match n % 100 {
            1 => Intcode::Add(pm_1, pm_2),
            2 => Intcode::Multiply(pm_1, pm_2),
            3 => Intcode::Input(pm_1),
            4 => Intcode::Output(pm_1),
            5 => Intcode::JumpIfTrue(pm_1, pm_2),
            6 => Intcode::JumpIfFalse(pm_1, pm_2),
            7 => Intcode::LessThan(pm_1, pm_2),
            8 => Intcode::Equals(pm_1, pm_2),
            99 => Intcode::Terminate,
            _ => panic!("Error: Invalid Intcode"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub memory: Vec<i32>,
    pub pointer: usize,
}

impl Program {
    pub fn new(program: Vec<i32>) -> Program {
        Program {
            memory: program,
            pointer: 0,
        }
    }

    pub fn new_from_file(filename: &str) -> Program {
        let raw_program: String = fs::read_to_string(filename).expect("Error reading file");
        let program: Vec<i32> = raw_program
            .trim()
            .split(',')
            .into_iter()
            .map(|n| n.parse::<i32>())
            .filter_map(Result::ok)
            .collect();

        Self::new(program)
    }

    pub fn run(&mut self, mut input: Option<i32>, signal: i32) -> Vec<Option<i32>> {
        let mut output: Vec<Option<i32>> = vec![];

        loop {
            let operation: Intcode = Intcode::from(self.memory[self.pointer]);

            match operation {
                Intcode::Add(mode_1, mode_2) => {
                    // ADD - opcode, read 1 index, read 2 index, write index
                    let value_1 = get(&self.memory, self.pointer, mode_1);
                    let value_2 = get(&self.memory, self.pointer + 1, mode_2);

                    set(&mut self.memory, self.pointer + 3, value_1 + value_2);

                    self.pointer += 4;
                }

                Intcode::Multiply(mode_1, mode_2) => {
                    // MULTIPLY - opcode, read 1 index, read 2 index, write index
                    let value_1 = get(&self.memory, self.pointer, mode_1);
                    let value_2 = get(&self.memory, self.pointer + 1, mode_2);

                    set(&mut self.memory, self.pointer + 3, value_1 * value_2);
                    self.pointer += 4;
                }

                Intcode::Input(_mode_1) => {
                    // INPUT - opcode, write index
                    // Opcode 3 takes a single integer as input and saves it to the
                    // position given by its only parameter. For example, the
                    // instruction 3,50 would take an input value and store it at
                    // address 50.
                    let i_1: usize = self.memory[self.pointer + 1] as usize;

                    match input.take() {
                        Some(value) => {
                            // TODO: Why can't I do: set(&mut self.memory, i_1, value);
                            self.memory[i_1] = value;
                        }
                        None => {
                            self.memory[i_1] = signal;
                        }
                    }
                    self.pointer += 2;
                }

                Intcode::Output(mode_1) => {
                    // OUTPUT - opcode, read index
                    // Opcode 4 outputs the value of its only parameter. For
                    // example, the instruction 4,50 would output the value at
                    // address 50.
                    let value_1 = get(&self.memory, self.pointer, mode_1);

                    self.pointer += 2;

                    output.push(Some(value_1));
                }

                Intcode::JumpIfTrue(mode_1, mode_2) => {
                    // Jump-if-true: if the first parameter is non-zero, it
                    // sets the instruction pointer to the value from the second
                    // parameter. Otherwise, it does nothing.
                    let value_1 = get(&self.memory, self.pointer, mode_1);
                    let value_2 = get(&self.memory, self.pointer + 1, mode_2);

                    if value_1 != 0 {
                        self.pointer = value_2 as usize;
                    } else {
                        self.pointer += 3;
                    }
                }

                Intcode::JumpIfFalse(mode_1, mode_2) => {
                    // Jump-if-false: if the first parameter is zero, it
                    // sets the instruction pointer to the value from the second
                    // parameter. Otherwise, it does nothing.
                    let value_1 = get(&self.memory, self.pointer, mode_1);
                    let value_2 = get(&self.memory, self.pointer + 1, mode_2);

                    if value_1 == 0 {
                        self.pointer = value_2 as usize;
                    } else {
                        self.pointer += 3;
                    }
                }

                Intcode::LessThan(mode_1, mode_2) => {
                    // Less than: if the first parameter is less than the
                    // second parameter, it stores 1 in the position given by the
                    // third parameter. Otherwise, it stores 0.
                    let value_1 = get(&self.memory, self.pointer, mode_1);
                    let value_2 = get(&self.memory, self.pointer + 1, mode_2);

                    if value_1 < value_2 {
                        set(&mut self.memory, self.pointer + 3, 1);
                    } else {
                        set(&mut self.memory, self.pointer + 3, 0);
                    }
                    self.pointer += 4;
                }

                Intcode::Equals(mode_1, mode_2) => {
                    // Equals: if the first parameter is equal to the second
                    // parameter, it stores 1 in the position given by the third
                    // parameter. Otherwise, it stores 0.
                    let value_1 = get(&self.memory, self.pointer, mode_1);
                    let value_2 = get(&self.memory, self.pointer + 1, mode_2);

                    if value_1 == value_2 {
                        set(&mut self.memory, self.pointer + 3, 1);
                    } else {
                        set(&mut self.memory, self.pointer + 3, 0);
                    }
                    self.pointer += 4;
                }

                Intcode::Terminate => {
                    break;
                }
            }
        }
        output
    }
}

#[test]
fn test_day_5_intcode_1() {
    let input: Option<i32> = Some(17);
    let mut program: Program = Program::new(vec![3, 0, 4, 0, 99]);

    assert_eq!(17, program.run(input, 0).last().unwrap().unwrap());
}

#[test]
fn test_day_5_intcode_2() {
    let input: Option<i32> = None;
    let mut program: Program = Program::new(vec![1002, 4, 3, 4, 33]);
    let expected: Vec<i32> = vec![1002, 4, 3, 4, 99];

    program.run(input, 0);

    assert_eq!(expected, program.memory);
}

#[test]
fn test_day_5_intcode_3() {
    let input: Option<i32> = None;
    let mut program: Program = Program::new(vec![1101, 100, -1, 4, 0]);
    let expected: Vec<i32> = vec![1101, 100, -1, 4, 99];

    program.run(input, 0);

    assert_eq!(expected, program.memory);
}

#[test]
fn test_day_5_intcode_4_equal() {
    // output 1 if input == 8, otherwise 0 (using position mode)
    let mut program: Program = Program::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);

    assert_eq!(1, program.run(Some(8), 0).last().unwrap().unwrap());
}

#[test]
fn test_day_5_intcode_4_not_equal() {
    // output 1 if input == 8, otherwise 0 (using position mode)
    let mut program: Program = Program::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);

    assert_eq!(0, program.run(Some(3), 0).last().unwrap().unwrap());
}

#[test]
fn test_day_5_intcode_5_equal() {
    // output 1 if input == 8, otherwise 0 (using immediate mode)
    let mut program: Program = Program::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);

    assert_eq!(1, program.run(Some(8), 0).last().unwrap().unwrap());
}

#[test]
fn test_day_5_intcode_5_not_equal() {
    // output 1 if input == 8, otherwise 0 (using immediate mode)
    let mut program: Program = Program::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);

    assert_eq!(0, program.run(Some(3), 0).last().unwrap().unwrap());
}

#[test]
fn test_day_5_intcode_6_less_than() {
    // output 1 if input < 8, otherwise 0 (using position mode)
    let mut program: Program = Program::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);

    assert_eq!(1, program.run(Some(7), 0).last().unwrap().unwrap());
}

#[test]
fn test_day_5_intcode_6_greater_than() {
    // output 1 if input < 8, otherwise 0 (using position mode)
    let mut program: Program = Program::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);

    assert_eq!(0, program.run(Some(9), 0).last().unwrap().unwrap());
}

#[test]
fn test_day_5_intcode_7_less_than() {
    // output 1 if input < 8, otherwise 0 (using immediate mode)
    let mut program: Program = Program::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);

    assert_eq!(1, program.run(Some(7), 0).last().unwrap().unwrap());
}

#[test]
fn test_day_5_intcode_7_greater_than() {
    // output 1 if input < 8, otherwise 0 (using immediate mode)
    let mut program: Program = Program::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);

    assert_eq!(0, program.run(Some(9), 0).last().unwrap().unwrap());
}

#[test]
fn test_day_5_intcode_8_equal() {
    // output 0 if input == 0, otherwise 1 (using position mode)
    let mut program: Program = Program::new(vec![
        3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
    ]);

    assert_eq!(0, program.run(Some(0), 0).last().unwrap().unwrap());
}

#[test]
fn test_day_5_intcode_8_not_equal() {
    // output 0 if input == 0, otherwise 1 (using position mode)
    let mut program: Program = Program::new(vec![
        3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
    ]);

    assert_eq!(1, program.run(Some(7), 0).last().unwrap().unwrap());
}

#[test]
fn test_day_5_intcode_9_equal() {
    // output 0 if input == 0, otherwise 1 (using immediate mode)
    let mut program: Program = Program::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);

    assert_eq!(1, program.run(Some(7), 0).last().unwrap().unwrap());
}

#[test]
fn test_day_5_intcode_9_not_equal() {
    // output 0 if input == 0, otherwise 1 (using immediate mode)
    let mut program: Program = Program::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);

    assert_eq!(0, program.run(Some(0), 0).last().unwrap().unwrap());
}

#[test]
fn test_day_5_intcode_10_less_than() {
    // output 999 if input < 8; 1000 if input == 8; 1001 if input > 8
    let mut program: Program = Program::new(vec![
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ]);

    assert_eq!(999, program.run(Some(7), 0).last().unwrap().unwrap());
}

#[test]
fn test_day_5_intcode_10_equal() {
    // output 999 if input < 8; 1000 if input == 8; 1001 if input > 8
    let mut program: Program = Program::new(vec![
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ]);

    assert_eq!(1000, program.run(Some(8), 0).last().unwrap().unwrap());
}

#[test]
fn test_day_5_intcode_10_greater_than() {
    // output 999 if input < 8; 1000 if input == 8; 1001 if input > 8
    let mut program: Program = Program::new(vec![
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ]);

    assert_eq!(1001, program.run(Some(10), 0).last().unwrap().unwrap());
}

#[test]
fn test_exercise1() {
    assert_eq!(8332629, exercise_1());
}

#[test]
fn test_exercise2() {
    assert_eq!(8805067, exercise_2());
}
