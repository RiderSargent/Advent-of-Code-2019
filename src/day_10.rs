use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
struct Star {
    r: i32,
    c: i32,
    visible: usize,
}

impl Star {
    fn new(r: i32, c: i32) -> Star {
        Star {
            r: r,
            c: c,
            visible: 0,
        }
    }

    fn set_visible(&mut self, v: usize) {
        self.visible = v;
    }
}

impl PartialEq for Star {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.c == other.c
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Direction {
    half: Half,
    slope: i32,
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
enum Half {
    UpperLeft,
    LowerRight,
}

fn get_data() -> Vec<Vec<i32>> {
    let data = std::fs::read_to_string("input_day_10.txt").expect("Error reading file");
    let data: Vec<Vec<i32>> = data
        .trim()
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect::<Vec<i32>>()
        .chunks(27)
        .map(|row| row.to_vec())
        .collect();

    data
}

fn get_stars_from_data(data: Vec<Vec<i32>>) -> Vec<Star> {
    let mut stars: Vec<Star> = vec![];
    let height = data.len();
    let width = data[0].len();

    for r in 0..height {
        for c in 0..width {
            if data[r][c] == 1 {
                stars.push(Star::new(r as i32, c as i32));
            }
        }
    }

    stars
}

fn get_visibles(stars: Vec<Star>) -> Vec<Star> {
    // .#..#   .7..7
    // .....   .....
    // #####   67775
    // ....#   ....7
    // ...##   ...87

    let l_stars = stars.clone();
    for mut s in l_stars {
        println!("-> {:?}, {:?}", s.r, s.c);

        let mut slopes: HashSet<Direction> = HashSet::new();
        let other_stars: Vec<Star> = stars
            .clone()
            .iter()
            .filter(|other| other != &&s)
            .cloned()
            .collect();

        for o in other_stars {
            let rise = (s.r - o.r) as f32;
            let run = (s.c - o.c) as f32;
            let direction = if s.c == o.c {
                if rise > 0.0 {
                    Direction {half: Half::UpperLeft, slope: std::f32::MAX.round() as i32}
                } else {
                    Direction {half: Half::LowerRight, slope: std::f32::MIN.round() as i32}
                }
            } else {
                let slope = (rise / run * -100000.0).round() as i32;
                if rise > 0.0 {
                    Direction {half: Half::UpperLeft, slope: slope}
                } else {
                    Direction {half: Half::LowerRight, slope: slope}
                }
            };

            slopes.insert(direction);
            s.set_visible(slopes.len());
            println!("{:?}, {:?}: {:?}", o.r, o.c, direction);
        }
        println!("");
    }

    vec![]
}

fn exercise_1() -> i32 {
    let data: Vec<Vec<i32>> = get_data();

    let height = data.len();
    let width = data[0].len();

    for r in 0..height {
        for c in 0..width {
            print!("{:?}", data[r][c]);
        }
        println!("");
    }

    0
}

#[test]
fn test_get_stars_from_data() {
    let data = vec![
        vec![0, 1, 0, 0, 1],
        vec![0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1],
        vec![0, 0, 0, 0, 1],
        vec![0, 0, 0, 1, 1],
    ];

    let expected = vec![
        Star { r: 0, c: 1, visible: 0 },
        Star { r: 0, c: 4, visible: 0 },
        Star { r: 2, c: 0, visible: 0 },
        Star { r: 2, c: 1, visible: 0 },
        Star { r: 2, c: 2, visible: 0 },
        Star { r: 2, c: 3, visible: 0 },
        Star { r: 2, c: 4, visible: 0 },
        Star { r: 3, c: 4, visible: 0 },
        Star { r: 4, c: 3, visible: 0 },
        Star { r: 4, c: 4, visible: 0 },
    ];

    assert_eq!(expected, get_stars_from_data(data));
}

#[ignore]
#[test]
fn test_get_visibles() {
    let stars = vec![
        Star { r: 0, c: 1, visible: 0 },
        Star { r: 0, c: 4, visible: 0 },
        Star { r: 2, c: 0, visible: 0 },
        Star { r: 2, c: 1, visible: 0 },
        Star { r: 2, c: 2, visible: 0 },
        Star { r: 2, c: 3, visible: 0 },
        Star { r: 2, c: 4, visible: 0 },
        Star { r: 3, c: 4, visible: 0 },
        Star { r: 4, c: 3, visible: 0 },
        Star { r: 4, c: 4, visible: 0 },
    ];

    let expected = vec![
        Star { r: 0, c: 1, visible: 7 },
        Star { r: 0, c: 4, visible: 7 },
        Star { r: 2, c: 0, visible: 6 },
        Star { r: 2, c: 1, visible: 7 },
        Star { r: 2, c: 2, visible: 7 },
        Star { r: 2, c: 3, visible: 7 },
        Star { r: 2, c: 4, visible: 5 },
        Star { r: 3, c: 4, visible: 7 },
        Star { r: 4, c: 3, visible: 8 },
        Star { r: 4, c: 4, visible: 7 },
    ];

    assert_eq!(expected, get_visibles(stars));
}


fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[test]
fn test_gcd() {
    assert_eq!(5, gcd(15, 20));
    assert_eq!(3, gcd(3, 6));
}

fn simplify_fraction(numerator: i32, denominator: i32) -> (i32, i32) {
    let gcd: i32 = gcd(numerator, denominator);

    (numerator / gcd, denominator / gcd)
}

#[test]
fn test_simplify_fraction() {
    assert_eq!((1, 5), simplify_fraction(3, 15));
    assert_eq!((2, 3), simplify_fraction(10, 15));
    assert_eq!((7, 8), simplify_fraction(49, 56));
}
