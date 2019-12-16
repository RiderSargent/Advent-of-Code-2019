pub fn exercise_1() {
    // a few key facts about the password:
    // It is a six-digit number.
    // The value is within the range given in your puzzle input.

    // Two adjacent digits are the same (like 22 in 122345).
    // Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).

    // 111111 meets these criteria (double 11, never decreases).
    // 223450 does not meet these criteria (decreasing pair of digits 50).
    // 123789 does not meet these criteria (no double).

    // input range: 134792-675810
    let candidates: Vec<u32> = (134792..=675810)
        .into_iter()
        .filter(|c| password_filter(&c))
        .collect::<Vec<u32>>();

    // println!("{:?}", candidates);
    // println!("\nFirst: {}", candidates[0]);
    // println!("Last: {}\n", candidates[candidates.len() - 1]);

    println!("[D4E1] number of potential passwords: {}", candidates.len());
}

fn password_filter(pw: &u32) -> bool {
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
    // for i in 0..5 {
    //     if digits[i] == digits[i+1] {
    //         has_double = true;
    //         break;
    //     }
    // }

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
