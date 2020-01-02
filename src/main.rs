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
    // day_3::exercise_1();
    // day_3::exercise_2();
    // day_4::exercise_1();
    // day_4::exercise_2();
    // day_5::exercise_1();
    // day_5::exercise_2();
    // day_6::exercise_1();
    // day_6::exercise_2();
}
