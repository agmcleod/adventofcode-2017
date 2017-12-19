fn get_buffer(size: usize) -> Vec<usize> {
    let mut buffer = Vec::with_capacity(size);
    buffer.push(0);
    buffer
}

fn process_buffer(buffer: &mut Vec<usize>, position: &mut usize, step: usize, iteration: usize) {
    *position = (step + *position) % buffer.len() + 1;
    buffer.insert(*position, iteration);
}

fn main() {
    let step = 345;
    let mut buffer = get_buffer(2018);

    let mut position = 0;
    for n in 1..2018 {
        process_buffer(&mut buffer, &mut position, step, n);
    }
    println!("{}", buffer[(position + 1) % buffer.len()]);

    let mut zero_position = 0;
    let mut after_zero = 0;
    let mut position = 0;
    let size = 50_000_000;
    for n in 1..size {
        position = (step + position) % n + 1;
        if position == 0 {
            zero_position += 1;
        } else if position == zero_position + 1 {
            after_zero = n;
        }
    }

    println!("{}", after_zero);
}