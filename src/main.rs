mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod intcode;
mod tests;

fn main() {
    println!("[D1E1] Total fuel: {}", day_1::exercise_1());
    println!("[D1E2] Total fuel: {}", day_1::exercise_2());
    println!("[D2E1] Value at position 0: {:?}", day_2::exercise_1());
    println!("[D2E2] 100 * noun + verb: {:?}", day_2::exercise_2());
    println!("[D3E1] distance: {:?}", day_3::exercise_1());
    println!("[D3E2] min_path_length: {:?}", day_3::exercise_2());
    println!("[D4E1] number of potential passwords: {}", day_4::exercise_1());
    println!("[D4E2] number of potential passwords: {}", day_4::exercise_2());
    day_5::exercise_1();
    day_5::exercise_2();
    println!("[D6E1] Number of orbits: {}", day_6::exercise_1());
    println!("[D6E2] shortest orbital transfer path length: {:?}", day_6::exercise_2());
}
