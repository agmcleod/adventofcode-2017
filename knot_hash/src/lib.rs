pub fn build_numbers(size: usize) -> Vec<usize> {
    let mut numbers = Vec::with_capacity(size);
    for n in 0..size {
        numbers.push(n);
    }

    numbers
}

pub fn create(value: &str, size: usize) -> Vec<usize> {
    let mut lengths = value.as_bytes().iter().map(|n| *n as usize).collect::<Vec<usize>>();
    lengths.append(&mut vec![17, 31, 73, 47, 23]);

    let mut skip_size = 0;
    let mut current_index = 0;
    let mut numbers = build_numbers(size);

    for _ in 0..64 {
        run_iteration(&mut numbers, size, &lengths, &mut current_index, &mut skip_size);
    }

    numbers.chunks(16).map(|set| {
        set.iter().fold(0, |sum, n| sum ^ n)
    }).collect()
}

pub fn numbers_to_hash(numbers: &Vec<usize>) -> String {
    numbers.iter().map(|n| format!("{:x}", n)).collect()
}

pub fn run_iteration(numbers: &mut Vec<usize>, size: usize, lengths: &[usize], current_index: &mut usize, skip_size: &mut usize) {
    for length in lengths {
        if *length > numbers.len() {
            continue
        }

        if *length != 0 {
            let mut section: Vec<usize> = Vec::with_capacity(*length);
            for i in 0..*length {
                section.push(*numbers.get((i + *current_index) % size).unwrap());
            }

            section.reverse();
            for (i, n) in section.iter().enumerate() {
                numbers[(i + *current_index) % size] = *n;
            }
        }

        *current_index += *length + *skip_size;

        *skip_size += 1;
    }
}