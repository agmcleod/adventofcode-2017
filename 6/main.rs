use std::collections::HashMap;

fn get_index_for_largest(input: &[usize]) -> usize {
    let mut result_index = 0;
    let mut maximum = 0;
    for (index, value) in input.iter().enumerate() {
        if *value > maximum {
            maximum = *value;
            result_index = index;
        }
    }

    result_index
}

fn distribute_from_index(input: &mut [usize], index: &usize) {
    let value = input[*index];
    input[*index] = 0;
    let len = input.len();
    for i in 0..value {
        input[(index + 1 + i) % len] += 1;
    }
}

fn create_configuration(input: &[usize]) -> String {
    input.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(",")
}

fn main() {
    let mut input = [2, 8, 8, 5, 4, 2, 3, 1, 5, 5, 1, 2, 15, 13, 5, 14];

    let mut configurations: HashMap<String, usize> = HashMap::new();
    let mut count = 0;

    loop {
        let index = get_index_for_largest(&input);
        distribute_from_index(&mut input, &index);

        let config = create_configuration(&input);
        count += 1;
        if configurations.contains_key(&config) {
            println!("count: {}, cycles since last: {}", count, count - configurations[&config]);
            break
        }
        configurations.insert(config, count);
    }
}