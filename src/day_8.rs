pub fn exercise_1() -> usize {
    let data = std::fs::read_to_string("input_day_08.txt").expect("Error reading file");
    let data: Vec<i32> = data
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<i32>().unwrap())
        .collect();

    let mut layers: Vec<Vec<i32>> = vec![];
    for row in data.chunks(150) {
        layers.push(row.to_vec());
    }

    let mut min_zeros = std::usize::MAX;
    let mut calc: usize = 0;

    for r in 0..100 {
        let num_zeros = layers[r].clone().into_iter().filter(|d| d == &0).collect::<Vec<i32>>().len();
        let num_ones  = layers[r].clone().into_iter().filter(|d| d == &1).collect::<Vec<i32>>().len();
        let num_twos  = layers[r].clone().into_iter().filter(|d| d == &2).collect::<Vec<i32>>().len();

        if num_zeros < min_zeros {
            min_zeros = num_zeros;
            calc = num_ones * num_twos;
        }
    }

    calc
}

#[test]
fn test_exercise1() {
    assert_eq!(1463, exercise_1());
}


