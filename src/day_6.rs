use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io::{prelude::*, BufReader};

pub fn exercise_1() -> u32 {
    let objects: HashMap<String, Option<String>> = load_input("input_day_06.txt");
    let mut count: u32 = 0;

    for (object, _orbits) in &objects {
        count += count_orbits(&objects, object.to_string());
    }

    count
}

pub fn exercise_2() -> usize {
    let objects: HashMap<String, Option<String>> = load_input("input_day_06.txt");

    let my_parents: HashSet<_> = get_parents(&objects, "YOU".to_string())
        .iter()
        .cloned()
        .collect();
    let santa_parents: HashSet<_> = get_parents(&objects, "SAN".to_string())
        .iter()
        .cloned()
        .collect();

    my_parents
        .symmetric_difference(&santa_parents)
        .collect::<HashSet<_>>()
        .len()
}

fn get_parents(objects: &HashMap<String, Option<String>>, current_object: String) -> Vec<String> {
    match objects.get(&current_object) {
        None => vec![],
        Some(optional_obj) => match optional_obj {
            None => vec![],
            Some(parent_obj) => [
                get_parents(objects, parent_obj.to_string()),
                vec![parent_obj.to_string()],
            ]
            .concat(),
        },
    }
}

fn count_orbits(objects: &HashMap<String, Option<String>>, current_object: String) -> u32 {
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

#[test]
fn test_exercise1() {
    assert_eq!(294191, exercise_1());
}

#[test]
fn test_exercise2() {
    assert_eq!(424, exercise_2());
}
