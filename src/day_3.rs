use std::collections::HashSet;
use std::fs;
use std::hash::{Hash, Hasher};
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

    // distance: 6 -> to (-3, 3)
    // path length: 30 -> to (-5, 6)
    routes = vec![
        vec!["R8", "U5", "L5", "D3"].into_iter().map(|d| d.to_string()).collect(),
        vec!["U7", "R6", "D4", "L4"].into_iter().map(|d| d.to_string()).collect(),
    ];

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

    let r1_points: Vec<(i32, i32)> = get_points(routes[0].to_owned());
    let r2_points: Vec<(i32, i32)> = get_points(routes[1].to_owned());

    print_points("r1_points", &r1_points);
    print_points("r2_points", &r2_points);

    // Need to clone points iter into HashSet
    // HashSet::from_iter(points.iter().cloned())

    // let intersection = r1_points.intersection(&r2_points);

    // let min_distance: i32 = std::i32::MAX;

    // let mut l1: usize;
    // let mut l2: usize;
    // let mut min_path_length: usize = std::usize::MAX;

    // println!("\nintersection:");
    // for p in intersection {
    //     if p.row != 0 || p.col != 0 {
    //         // println!("  {:?}", p);
    //         // if p.distance < min_distance {
    //         //     min_distance = p.distance;
    //         // }
    //         l1 = r1_points.into_iter().position(|q| q.row == p.row && q.col == p.col).unwrap();
    //         l2 = r2_points.into_iter().position(|q| q.row == p.row && q.col == p.col).unwrap();
    //         let combined_path_length = l1 + l2;
    //         if combined_path_length < min_path_length {
    //             min_path_length = combined_path_length;
    //         }
    //     }
    // }
    // println!("");

    // println!("[D3E1] distance: {:?}", min_distance);
}

fn get_points(route: Vec<String>) -> Vec<(i32, i32)> {
    let mut points: Vec<(i32, i32)> = vec![(0, 0)];

    for line in route {
        let chars: Vec<char> = line.chars().collect();
        let direction: &char = chars.first().unwrap();
        let distance: i32 = chars[1..]
            .to_owned()
            .into_iter()
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
        let (r, c) = *points.last().unwrap();

        match direction {
            'U' => {
                for i in 1..=distance {
                    points.push((r - i, c));
                }
            }
            'D' => {
                for i in 1..=distance {
                    points.push((r + i, c));
                }
            }
            'L' => {
                for i in 1..=distance {
                    points.push((r, c - i,));
                }
            }
            'R' => {
                for i in 1..=distance {
                    points.push((r, c + i));
                }
            }
            _ => panic!("Bad data: Illegal direction."),
        }
    }

    points
}

fn print_points(name: &str, points: &Vec<(i32, i32)>) {
    println!("\n{}:", name);
    for i in 0..points.len() {
        println!("{:2}:  {:?}", i, points[i]);
    }
}
