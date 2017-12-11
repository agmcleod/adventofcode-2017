const LENGTH: usize = 256;

fn build_numbers() -> Vec<usize> {
    let mut numbers = Vec::with_capacity(LENGTH);
    for n in 0..LENGTH {
        numbers.push(n);
    }

    numbers
}

fn run_iteration(numbers: &mut Vec<usize>, lengths: &[usize], current_index: &mut usize, skip_size: &mut usize) {
    for length in lengths {
        if *length > numbers.len() {
            continue
        }

        if *length != 0 {
            let mut section: Vec<usize> = Vec::with_capacity(*length);
            for i in 0..*length {
                section.push(*numbers.get((i + *current_index) % LENGTH).unwrap());
            }

            section.reverse();
            for (i, n) in section.iter().enumerate() {
                numbers[(i + *current_index) % LENGTH] = *n;
            }
        }

        *current_index += *length + *skip_size;

        *skip_size += 1;
    }
}

fn main() {
    let lengths = [129,154,49,198,200,133,97,254,41,6,2,1,255,0,191,108];

    let mut numbers = build_numbers();
    run_iteration(&mut numbers, &lengths, &mut 0, &mut 0);
    println!("{}", numbers[0] * numbers[1]);

    let mut numbers = build_numbers();
    let lengths: Vec<String> = lengths.iter().map(|n| format!("{}", n)).collect();
    let mut lengths = lengths.join(",").as_bytes().iter().map(|n| *n as usize).collect::<Vec<usize>>();

    lengths.append(&mut vec![17, 31, 73, 47, 23]);

    let mut skip_size = 0;
    let mut current_index = 0;
    for _ in 0..64 {
        run_iteration(&mut numbers, &lengths, &mut current_index, &mut skip_size);
    }

    let hash: Vec<String> = numbers.chunks(16).map(|set| {
        format!("{:x}", set.iter().fold(0, |sum, n| sum ^ n))
    }).collect();

    println!("{:?}", hash.join(""));
}