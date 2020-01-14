use std::fs;
use itertools::Itertools;

pub fn exercise_1() -> i32 {
    let permutations = (0..5).permutations(5);
    let amp: Program = Program::new_from_file("input_day_07.txt");
    let mut max = 0;

    for p in permutations {
        let output = run_amps_1(&p, amp.clone());
        if output > max {
            max = output;
        }
    }

    max
}

pub fn exercise_2() -> i32 {
    let permutations = (5..10).permutations(5);
    let amp: Program = Program::new_from_file("input_day_07.txt");
    let mut max = 0;

    for p in permutations {
        let output = run_amps_2(&p, amp.clone());
        if output > max {
            max = output;
        }
    }

    max
}

fn run_amps_1(phase_settings: &Vec<i32>, amp: Program) -> i32 {
    let phase_settings: Vec<i32> = phase_settings.clone();

    let mut signal: i32 = 0;

    for s in phase_settings {
        signal = amp.clone().run(Some(s), signal).unwrap();
    }

    signal
}

fn run_amps_2(phase_settings: &Vec<i32>, master_amp: Program) -> i32 {
    let amp_1: Program = master_amp.clone();
    let amp_2: Program = master_amp.clone();
    let amp_3: Program = master_amp.clone();
    let amp_4: Program = master_amp.clone();
    let amp_5: Program = master_amp.clone();

    let mut amps = [amp_1, amp_2, amp_3, amp_4, amp_5];

    let mut phase_settings = phase_settings.iter().cloned();
    let mut signal: i32 = 0;

    for i in (0..5).cycle() {
        if let Some(s) = amps[i].run(phase_settings.next(), signal) {
            signal = s;
        } else {
            break
        }
    }

    signal
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

    pub fn run(&mut self, mut input: Option<i32>, signal: i32) -> Option<i32> {
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
                        },
                        None => {
                            self.memory[i_1] = signal;
                        },
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

                    return Some(value_1);
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
        None
    }
}

#[test]
fn test_run_amps_1_a() {
    let amp: Program = Program::new(vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]);
    let phase_settings: Vec<i32> = vec![4,3,2,1,0];

    assert_eq!(43210, run_amps_1(&phase_settings, amp));
}

#[test]
fn test_run_amps_1_b() {
    let amp: Program = Program::new(vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0]);
    let phase_settings: Vec<i32> = vec![0,1,2,3,4];

    assert_eq!(54321, run_amps_1(&phase_settings, amp));
}

#[test]
fn test_run_amps_1_c() {
    let amp: Program = Program::new(vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0]);
    let phase_settings: Vec<i32> = vec![1,0,4,3,2];

    assert_eq!(65210, run_amps_1(&phase_settings, amp));
}

#[test]
fn test_run_amps_2_a() {
    let amp: Program = Program::new(vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5]);
    let phase_settings: Vec<i32> = vec![9,8,7,6,5];

    assert_eq!(139629729, run_amps_2(&phase_settings, amp));
}

#[test]
fn test_run_amps_2_b() {
    let amp: Program = Program::new(vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10]);
    let phase_settings: Vec<i32> = vec![9,7,8,5,6];

    assert_eq!(18216, run_amps_2(&phase_settings, amp));
}

#[test]
fn test_exercise1() {
    assert_eq!(24625, exercise_1());
}

#[test]
fn test_exercise2() {
    assert_eq!(36497698, exercise_2());
}

