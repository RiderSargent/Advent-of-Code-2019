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

    routes = vec![
        vec!["R8".to_string(),"U5".to_string(),"L5".to_string(),"D3".to_string()],
        vec!["U7".to_string(),"R6".to_string(),"D4".to_string(),"L4".to_string()],
    ];

    println!("{:?}", routes[0]);
    println!("{:?}\n", routes[1]);

    let r1_points = get_points(routes[0].to_owned());
    let r2_points = get_points(routes[1].to_owned());

    println!("{:?}", r1_points);
    println!("{:?}\n", r2_points);

    let intersection = r1_points.intersection(&r2_points);

    println!("Intersection: {:?}", intersection);

    let manhattan_distances: Vec<i32> = intersection.map(|(i, j)| manhattan_distance((i, j))).collect();

    println!("manhattan_distances: {:?}", manhattan_distances);

    println!("sorted intersection: {:?}", intersection.collect::<Vec<(i32, i32)>>().sort_by(|a, b| manhattan_distance(a) < manhattan_distance(b)));
    // println!("[D3E1] routes: {:?}", routes);
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

fn manhattan_distance(point: (&i32, &i32)) -> i32 {
    let (r, c) = point;

    r.abs() + c.abs()
}
