use std::collections::HashSet;
use std::fs;
use std::hash::Hash;
use std::io::{prelude::*, BufReader};
use std::iter::FromIterator;

pub fn exercise_1() -> i32 {
    let routes: Vec<Vec<String>> = get_routes();

    let r1_points: Vec<Point> = get_points(routes[0].to_owned());
    let r2_points: Vec<Point> = get_points(routes[1].to_owned());

    let r1_points_set: HashSet<Point> = HashSet::from_iter(r1_points.iter().cloned());
    let r2_points_set: HashSet<Point> = HashSet::from_iter(r2_points.iter().cloned());

    let intersection = r1_points_set.intersection(&r2_points_set);

    let mut min_distance: i32 = std::i32::MAX;

    for Point { x: r, y: c } in intersection.clone() {
        if r != &0 || c != &0 {
            let p_distance = r.abs() + c.abs();
            if p_distance < min_distance {
                min_distance = p_distance;
            }
        }
    }

    min_distance
}

pub fn exercise_2() -> usize {
    let routes: Vec<Vec<String>> = get_routes();

    let r1_points: Vec<Point> = get_points(routes[0].to_owned());
    let r2_points: Vec<Point> = get_points(routes[1].to_owned());

    let r1_points_set: HashSet<Point> = HashSet::from_iter(r1_points.iter().cloned());
    let r2_points_set: HashSet<Point> = HashSet::from_iter(r2_points.iter().cloned());

    let intersection = r1_points_set.intersection(&r2_points_set);

    let mut l1: usize;
    let mut l2: usize;
    let mut min_path_length: usize = std::usize::MAX;

    for Point { x: r, y: c } in intersection.clone() {
        if r != &0 || c != &0 {
            l1 = r1_points
                .clone()
                .into_iter()
                .position(|Point { x: a, y: b }| a == *r && b == *c)
                .unwrap();
            l2 = r2_points
                .clone()
                .into_iter()
                .position(|Point { x: a, y: b }| a == *r && b == *c)
                .unwrap();
            let combined_path_length = l1 + l2;
            if combined_path_length < min_path_length {
                min_path_length = combined_path_length;
            }
        }
    }

    min_path_length
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn get_routes() -> Vec<Vec<String>> {
    let mut routes: Vec<Vec<String>> = vec![];
    let filename = "input_day_03.txt";
    let file = fs::File::open(filename).expect("Error reading file");

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

    // --- Testing -------------------------------------------------------------
    // TODO: make these actual tests?
    // // distance: 6 -> to (-3, 3)
    // // path length: 30 -> to (-5, 6)
    // routes = vec![
    //     vec!["R8", "U5", "L5", "D3"].into_iter().map(|d| d.to_string()).collect(),
    //     vec!["U7", "R6", "D4", "L4"].into_iter().map(|d| d.to_string()).collect(),
    // ];

    // // distance: 159
    // // path length: 610
    // routes = vec![
    //     vec!["R75","D30","R83","U83","L12","D49","R71","U7","L72"].into_iter().map(|d| d.to_string()).collect(),
    //     vec!["U62","R66","U55","R34","D71","R55","D58","R83"].into_iter().map(|d| d.to_string()).collect(),
    // ];

    // // distance: 135
    // // path length: 410
    // routes = vec![
    //     vec!["R98","U47","R26","D63","R33","U87","L62","D20","R33","U53","R51"].into_iter().map(|d| d.to_string()).collect(),
    //     vec!["U98","R91","D20","R16","D67","R40","U7","R15","U6","R7"].into_iter().map(|d| d.to_string()).collect(),
    // ];

    routes
}

fn get_points(route: Vec<String>) -> Vec<Point> {
    let mut points: Vec<Point> = vec![Point { x: 0, y: 0 }];

    for line in route {
        let chars: Vec<char> = line.chars().collect();
        let direction: &char = chars.first().unwrap();
        let distance: i32 = chars[1..]
            .to_owned()
            .into_iter()
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
        let Point { x: r, y: c } = *points.last().unwrap();

        match direction {
            'U' => {
                for i in 1..=distance {
                    points.push(Point { x: r - i, y: c });
                }
            }
            'D' => {
                for i in 1..=distance {
                    points.push(Point { x: r + i, y: c });
                }
            }
            'L' => {
                for i in 1..=distance {
                    points.push(Point { x: r, y: c - i });
                }
            }
            'R' => {
                for i in 1..=distance {
                    points.push(Point { x: r, y: c + i });
                }
            }
            _ => panic!("Bad data: Illegal direction."),
        }
    }

    points
}

#[test]
fn test_exercise1() {
    assert_eq!(248, exercise_1());
}

#[test]
fn test_exercise2() {
    assert_eq!(28580, exercise_2());
}

