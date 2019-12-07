use std::collections::HashSet;
use std::iter::FromIterator;
use std::fs;
use std::io::{prelude::*, BufReader};

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
    // let mut closest: (&i32, &i32) = (&0, &0);
    for (i, j) in intersection {
        if i != &0 || j != &0 {
            let current_distance = manhattan_distance((i, j)); 
            if current_distance < min_distance {
                min_distance = current_distance;
                // closest = (i, j);
            }
        }
    }

    println!("[D3E1] distance: {:?}", min_distance);
}

fn get_points(route: Vec<String>) -> HashSet<(i32, i32)> {
    let mut points: Vec<(i32, i32)> = vec![(0, 0)];

    for line in route {
        let chars: Vec<char> = line.chars().collect();
        let direction = chars.first().unwrap();
        let distance: i32 = chars[1..].to_owned().into_iter().collect::<String>().parse::<i32>().unwrap();
        let (r, c) = *points.last().unwrap();

        match direction {
            'U' => {
                for i in 1..=distance {
                    points.push((r - i, c.to_owned()));
                }
            }
            'D' => {
                for i in 1..=distance {
                    points.push((r + i, c.to_owned()));
                }
            }
            'L' => {
                for i in 1..=distance {
                    points.push((r.to_owned(), c - i));
                }
            }
            'R' => {
                for i in 1..=distance {
                    points.push((r.to_owned(), c + i));
                }
            }
            _ => panic!("Bad data: Illegal direction."),
        }
    }

    HashSet::from_iter(points.iter().cloned())
}

fn manhattan_distance(point: (&i32, &i32)) -> u32 {
    let (r, c) = point;

    (r.abs() + c.abs()) as u32
}
