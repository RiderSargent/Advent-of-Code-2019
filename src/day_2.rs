use std::fs;

pub fn exercise_1() -> i32 {
    let mut program: Program = Program::new_from_file("input_day_02.txt");

    // "before running the program, replace position 1 with the value 12
    // and replace position 2 with the value 2."
    program.memory[1] = 12;
    program.memory[2] = 2;

    program.run(None, 0);

    program.memory[0]
}

pub fn exercise_2() -> i32 {
    let mut current_program: Program;
    let mut final_program: Program = Program::new(vec![]);
    let target: i32 = 19690720;

    'outer: for i in 0..100 {
        'inner: for j in 0..100 {
            current_program = Program::new_from_file("input_day_02.txt");
            current_program.memory[1] = i;
            current_program.memory[2] = j;

            current_program.run(None, 0);
            final_program = Program::new(current_program.memory);
            if final_program.memory[0] == target {
                break 'outer;
            }
        }
    }

    100 * final_program.memory[1] + final_program.memory[2]
}

enum Intcode {
    Add(i32, i32),
    Multiply(i32, i32),
    Terminate,
}

impl Intcode {
    fn from(n: i32) -> Intcode {
        let pm_1: i32 = (n / 100) % 10;
        let pm_2: i32 = (n / 1000) % 10;

        match n % 100 {
            1 => Intcode::Add(pm_1, pm_2),
            2 => Intcode::Multiply(pm_1, pm_2),
            99 => Intcode::Terminate,
            _ => panic!("Error: Invalid Intcode"),
        }
    }
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
        let mut i = 0;

        loop {
            let operation: Intcode = Intcode::from(self.memory[i]);

            match operation {
                Intcode::Add(mode_1, mode_2) => {
                    // ADD - opcode, read 1 index, read 2 index, write index
                    let value_1 = get(&self.memory, i, mode_1);
                    let value_2 = get(&self.memory, i + 1, mode_2);

                    set(&mut self.memory, i + 3, value_1 + value_2);

                    i += 4;
                }

                Intcode::Multiply(mode_1, mode_2) => {
                    // MULTIPLY - opcode, read 1 index, read 2 index, write index
                    let value_1 = get(&self.memory, i, mode_1);
                    let value_2 = get(&self.memory, i + 1, mode_2);

                    set(&mut self.memory, i + 3, value_1 * value_2);
                    i += 4;
                }

                Intcode::Terminate => {
                    break;
                }
            }
        }
        None
    }
}

