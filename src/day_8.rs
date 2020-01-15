const WIDTH: usize = 25;
const HEIGHT: usize = 6;

pub fn exercise_1() -> usize {
    let layers = get_layers();

    let mut min_zeros = std::usize::MAX;
    let mut calc: usize = 0;

    for r in 0..100 {
        let num_zeros = layers[r]
            .clone()
            .into_iter()
            .filter(|d| d == &0)
            .collect::<Vec<i32>>()
            .len();
        let num_ones = layers[r]
            .clone()
            .into_iter()
            .filter(|d| d == &1)
            .collect::<Vec<i32>>()
            .len();
        let num_twos = layers[r]
            .clone()
            .into_iter()
            .filter(|d| d == &2)
            .collect::<Vec<i32>>()
            .len();

        if num_zeros < min_zeros {
            min_zeros = num_zeros;
            calc = num_ones * num_twos;
        }
    }

    calc
}

pub fn exercise_2() -> Vec<i32> {
    let length = WIDTH * HEIGHT;
    let mut decoded: Vec<i32> = vec![0; length];
    let layers = get_layers();

    'decoded_loop: for i in 0..length {
        'layer_loop: for layer in &layers {
            if layer[i] != 2 {
                decoded[i] = layer[i];
                break 'layer_loop;
            }
        }
    }

    for row in decoded.chunks(WIDTH) {
        for pixel in row {
            if pixel == &0 {
                print!(" ");
            } else {
                print!("█");
            }
        }
        println!("");
    }

    println!("decoded: {:?}", decoded);

    decoded
}

fn get_layers() -> Vec<Vec<i32>> {
    let data = get_data();
    let mut layers: Vec<Vec<i32>> = vec![];

    for row in data.chunks(WIDTH * HEIGHT) {
        layers.push(row.to_vec());
    }

    layers
}

fn get_data() -> Vec<i32> {
    let data = std::fs::read_to_string("input_day_08.txt").expect("Error reading file");
    let data: Vec<i32> = data
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<i32>().unwrap())
        .collect();

    data
}

#[test]
fn test_exercise1() {
    assert_eq!(1463, exercise_1());
}

#[test]
fn test_exercise2() {
    let expected: Vec<i32> = vec![
        0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0,
        1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0,
        1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0,
        1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0,
        1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0,
        0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0
    ];

    assert_eq!(expected, exercise_2());
}
