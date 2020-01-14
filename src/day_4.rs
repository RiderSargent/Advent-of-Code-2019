pub fn exercise_1() -> usize {
    get_candidates_1(134792, 675810).len()
}

pub fn exercise_2() -> usize {
    get_candidates_2(134792, 675810).len()
}

// TODO: parameterize the filter function
fn get_candidates_1(start: u32, end: u32) -> Vec<u32> {
    (start..=end)
        .into_iter()
        .filter(|c| password_filter_1(&c))
        .collect::<Vec<u32>>()
}

fn password_filter_1(pw: &u32) -> bool {
    // TODO: extract digitization function
    // --- string digitization ---
    // let digits: Vec<u32> = n
    //     .to_owned()
    //     .to_string()
    //     .chars()
    //     .map(|c| c.to_digit(10).unwrap())
    //     .collect();

    // --- math digitization ---
    let mut digits: Vec<u32> = Vec::new();
    let mut n = pw;
    let mut foo;

    while n > &9 {
        digits.push(n % 10);
        foo = n / 10;
        n = &foo;
    }
    digits.push(*n);
    digits.reverse();

    // check increasing
    for i in 0..5 {
        if digits[i] > digits[i + 1] {
            return false;
        }
    }

    // check doubles
    let mut has_double: bool = false;
    for i in 0..5 {
        if digits[i] == digits[i + 1] {
            has_double = true;
            break;
        }
    }

    has_double
}

fn get_candidates_2(start: u32, end: u32) -> Vec<u32> {
    (start..=end)
        .into_iter()
        .filter(|c| password_filter_2(&c))
        .collect::<Vec<u32>>()
}

fn password_filter_2(pw: &u32) -> bool {
    // --- string digitization ---
    // let digits: Vec<u32> = n
    //     .to_owned()
    //     .to_string()
    //     .chars()
    //     .map(|c| c.to_digit(10).unwrap())
    //     .collect();

    // --- math digitization ---
    let mut digits: Vec<u32> = Vec::new();
    let mut n = pw;
    let mut foo;

    while n > &9 {
        digits.push(n % 10);
        foo = n / 10;
        n = &foo;
    }
    digits.push(*n);
    digits.reverse();

    // check increasing
    for i in 0..5 {
        if digits[i] > digits[i + 1] {
            return false;
        }
    }

    // check doubles
    let mut has_double: bool = false;

    if digits[0] == digits[1] && digits[0] != digits[2] {
        has_double = true;
    }

    if digits[1] == digits[2] && digits[1] != digits[0] && digits[1] != digits[3] {
        has_double = true;
    }

    if digits[2] == digits[3] && digits[2] != digits[1] && digits[2] != digits[4] {
        has_double = true;
    }

    if digits[3] == digits[4] && digits[3] != digits[2] && digits[3] != digits[5] {
        has_double = true;
    }

    if digits[4] == digits[5] && digits[4] != digits[3] {
        has_double = true;
    }

    has_double
}

#[test]
fn test_exercise1() {
    assert_eq!(1955, exercise_1());
}

#[test]
fn test_exercise2() {
    assert_eq!(1319, exercise_2());
}

