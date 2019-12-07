use std::collections::HashSet;
use std::fs;
use std::io::{prelude::*, BufReader};
use std::iter::FromIterator;

pub fn exercise_1() {
    let filename = "input_day_03.txt";
    let file = fs::File::open(filename).expect("Error reading file");
    let mut routes: Vec<Vec<String>> = vec![];

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let route: Vec<String> = line
            .unwrap()
            .split(',')
            .into_iter()
            .map(|i| i.to_string())
            .collect();

        routes.push(route.to_owned());
    }

    let r1_points = get_points(routes[0].to_owned());
    let r2_points = get_points(routes[1].to_owned());

    let intersection = r1_points.intersection(&r2_points);

    let mut min_distance: u32 = std::u32::MAX;
    for p in intersection {
        if p.row != 0 || p.col != 0 {
            if p.distance < min_distance {
                min_distance = p.distance;
            }
        }
    }

    println!("[D3E1] distance: {:?}", min_distance);
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    row: i32,
    col: i32,
    distance: u32,
}

impl Point {
    pub fn new(r: i32, c: i32) -> Self {
        Point { 
            row: r, 
            col: c,
            distance: (r.abs() + c.abs()) as u32,
        }
    }
}

fn get_points(route: Vec<String>) -> HashSet<Point> {
    let mut points: Vec<Point> = vec![Point::new(0, 0)];

    for line in route {
        let chars: Vec<char> = line.chars().collect();
        let direction: &char = chars.first().unwrap();
        let distance: i32 = chars[1..]
            .to_owned()
            .into_iter()
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
        let prev: Point = *points.last().unwrap();

        match direction {
            'U' => {
                for i in 1..=distance {
                    points.push(Point::new(prev.row - i, prev.col.to_owned()));
                }
            }
            'D' => {
                for i in 1..=distance {
                    points.push(Point::new(prev.row + i, prev.col.to_owned()));
                }
            }
            'L' => {
                for i in 1..=distance {
                    points.push(Point::new(prev.row.to_owned(), prev.col - i));
                }
            }
            'R' => {
                for i in 1..=distance {
                    points.push(Point::new(prev.row.to_owned(), prev.col + i));
                }
            }
            _ => panic!("Bad data: Illegal direction."),
        }
    }

    HashSet::from_iter(points.iter().cloned())
}
