use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
struct Star {
    r: i32,
    c: i32,
    index: usize,
    visible: usize,
}

impl Star {
    fn new(r: i32, c: i32, i: usize) -> Star {
        Star {
            r: r,
            c: c,
            index: i,
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
    slope: (i32, i32),
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
enum Half {
    UpperLeft,
    LowerRight,
}

fn exercise_1() -> usize {
    let data: Vec<Vec<i32>> = get_data();

    let stars = get_stars_from_data(data);
    let stars_2 = get_visibles(stars);
    let max = get_max_visible(stars_2);

    println!("Max: {}", max);

    max
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
    let mut index: usize = 0;

    for r in 0..height {
        for c in 0..width {
            if data[r][c] == 1 {
                stars.push(Star::new(r as i32, c as i32, index));
                index += 1;
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

    let mut l_stars = stars.clone();
    for i in 0..l_stars.len() {
        let mut slopes: HashSet<Direction> = HashSet::new();
        let other_stars: Vec<Star> = stars
            .clone()
            .iter()
            .filter(|o| o != &&l_stars[i])
            .cloned()
            .collect();

        for o in other_stars {
            let rise = l_stars[i].r - o.r;
            let run = l_stars[i].c - o.c;
            let half = if l_stars[i].index > o.index {
                Half::UpperLeft
            } else {
                Half::LowerRight
            };
            let slope = simplify_fraction(rise, run);
            let direction = Direction {
                half: half,
                slope: slope,
            };

            slopes.insert(direction);
            l_stars[i].set_visible(slopes.len());
        }
    }

    l_stars
}

fn get_max_visible(stars: Vec<Star>) -> usize {
    let mut max = 0;
    for i in 0..stars.len() {
        if stars[i].visible > max {
            max = stars[i].visible;
        }
    }

    max
}

fn simplify_fraction(numerator: i32, denominator: i32) -> (i32, i32) {
    let gcd: i32 = gcd(numerator, denominator);

    (numerator / gcd, denominator / gcd)
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
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
        Star { r: 0, c: 1, index: 0, visible: 0, },
        Star { r: 0, c: 4, index: 1, visible: 0, },
        Star { r: 2, c: 0, index: 2, visible: 0, },
        Star { r: 2, c: 1, index: 3, visible: 0, },
        Star { r: 2, c: 2, index: 4, visible: 0, },
        Star { r: 2, c: 3, index: 5, visible: 0, },
        Star { r: 2, c: 4, index: 6, visible: 0, },
        Star { r: 3, c: 4, index: 7, visible: 0, },
        Star { r: 4, c: 3, index: 8, visible: 0, },
        Star { r: 4, c: 4, index: 9, visible: 0, },
    ];

    assert_eq!(expected, get_stars_from_data(data));
}

#[test]
fn test_get_visibles() {
    let stars = vec![
        Star { r: 0, c: 1, index: 0, visible: 0, },
        Star { r: 0, c: 4, index: 1, visible: 0, },
        Star { r: 2, c: 0, index: 2, visible: 0, },
        Star { r: 2, c: 1, index: 3, visible: 0, },
        Star { r: 2, c: 2, index: 4, visible: 0, },
        Star { r: 2, c: 3, index: 5, visible: 0, },
        Star { r: 2, c: 4, index: 6, visible: 0, },
        Star { r: 3, c: 4, index: 7, visible: 0, },
        Star { r: 4, c: 3, index: 8, visible: 0, },
        Star { r: 4, c: 4, index: 9, visible: 0, },
    ];

    let expected = vec![
        Star { r: 0, c: 1, index: 0, visible: 7, },
        Star { r: 0, c: 4, index: 1, visible: 7, },
        Star { r: 2, c: 0, index: 2, visible: 6, },
        Star { r: 2, c: 1, index: 3, visible: 7, },
        Star { r: 2, c: 2, index: 4, visible: 7, },
        Star { r: 2, c: 3, index: 5, visible: 7, },
        Star { r: 2, c: 4, index: 6, visible: 5, },
        Star { r: 3, c: 4, index: 7, visible: 7, },
        Star { r: 4, c: 3, index: 8, visible: 8, },
        Star { r: 4, c: 4, index: 9, visible: 7, },
    ];

    assert_eq!(expected, get_visibles(stars));
}

#[test]
fn test_get_max_visible() {
    let stars = vec![
        Star { r: 0, c: 1, index: 0, visible: 7, },
        Star { r: 0, c: 4, index: 1, visible: 7, },
        Star { r: 2, c: 0, index: 2, visible: 6, },
        Star { r: 2, c: 1, index: 3, visible: 7, },
        Star { r: 2, c: 2, index: 4, visible: 7, },
        Star { r: 2, c: 3, index: 5, visible: 7, },
        Star { r: 2, c: 4, index: 6, visible: 5, },
        Star { r: 3, c: 4, index: 7, visible: 7, },
        Star { r: 4, c: 3, index: 8, visible: 8, },
        Star { r: 4, c: 4, index: 9, visible: 7, },
    ];

    let expected = 8;

    assert_eq!(expected, get_max_visible(stars));
}

#[test]
fn test_gcd() {
    assert_eq!(5, gcd(15, 20));
    assert_eq!(3, gcd(3, 6));
}

#[test]
fn test_simplify_fraction() {
    assert_eq!((1, 5), simplify_fraction(3, 15));
    assert_eq!((2, 3), simplify_fraction(10, 15));
    assert_eq!((7, 8), simplify_fraction(49, 56));
}

#[test]
fn test_exercise_1() {
    assert_eq!(296, exercise_1());
}
