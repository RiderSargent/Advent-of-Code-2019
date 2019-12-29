use std::collections::HashMap;
use std::fs;
use std::io::{prelude::*, BufReader};

pub fn exercise_1() {
    let objects: HashMap<String, Option<String>> = load_input("input_day_06.txt");
    let mut count: u32 = 0;

    // let mut objects: HashMap<String, Option<String>> = HashMap::new();
    // objects.insert("B".to_string(), Some("COM".to_string()));
    // objects.insert("C".to_string(), Some("B".to_string()));
    // objects.insert("D".to_string(), Some("C".to_string()));
    // objects.insert("E".to_string(), Some("D".to_string()));
    // objects.insert("F".to_string(), Some("E".to_string()));
    // objects.insert("G".to_string(), Some("B".to_string()));
    // objects.insert("H".to_string(), Some("G".to_string()));
    // objects.insert("I".to_string(), Some("D".to_string()));
    // objects.insert("J".to_string(), Some("E".to_string()));
    // objects.insert("K".to_string(), Some("J".to_string()));
    // objects.insert("L".to_string(), Some("K".to_string()));
    // above graph has 42 orbits

    for (object, _orbits) in &objects {
        count += count_orbits(&objects, object.to_string());
    }

    // should be 294191
    println!("Number of orbits: {}", count);
}

fn count_orbits(objects: &HashMap<String, Option<String>>, current_object: String) -> u32 {
    // println!("{:?} orbits {:?}", current_object, objects.get(&current_object).unwrap());
    match objects.get(&current_object) {
        None => return 0,
        Some(optional_obj) => match optional_obj {
            None => 0,
            Some(obj) => 1 + count_orbits(objects, obj.to_string()),
        },
    }
}

fn load_input(filename: &str) -> HashMap<String, Option<String>> {
    let file = fs::File::open(filename).expect("Error reading file");
    let reader = BufReader::new(file);
    let mut objects: HashMap<String, Option<String>> = HashMap::new();

    for line in reader.lines() {
        let unwrapped_line = line.unwrap();
        let object_data: Vec<&str> = unwrapped_line.split(')').into_iter().collect();
        let object_name: String = object_data[1].to_string();
        let object_orbits: Option<String> = Some(object_data[0].to_string());

        objects.insert(object_name.to_string(), object_orbits);
    }

    objects
}
