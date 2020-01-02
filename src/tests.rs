#[cfg(test)]

use super::*;

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

// #[ignore]
// #[test]
// fn test_d5e1() {
//     assert_eq!(1319, day_5::exercise_1());
// }

// #[ignore]
// #[test]
// fn test_d5e2() {
//     assert_eq!(1319, day_5::exercise_2());
// }

#[test]
fn test_d6e1() {
    assert_eq!(294191, day_6::exercise_1());
}

#[test]
fn test_d6e2() {
    assert_eq!(424, day_6::exercise_2());
}
