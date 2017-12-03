use std::cmp;

fn size_of_spiral_for_iteration(iteration: usize) -> usize {
    (iteration * 4 - 4)
}

fn main() {
    const FROM_POSITION: usize = 289326;

    let mut spiral_circle = 1;
    let mut current_position = 1;
    let mut spiral_count = 0;

    loop {
        spiral_circle += 2;
        let previous_position = current_position;
        current_position += size_of_spiral_for_iteration(spiral_circle);
        spiral_count += 1;
        if current_position > FROM_POSITION {
            let quarter_size = spiral_circle;
            let relative_position = FROM_POSITION - previous_position;

            let mut min = 1;
            let mut max = quarter_size;
            loop {
                if relative_position >= min && relative_position <= max {
                    let half = quarter_size / 2;
                    let min = if min != 1 {
                        min - 1
                    } else {
                        min
                    };
                    let distance_from_middle = cmp::max(min + half, relative_position) - cmp::min(min + half, relative_position);
                    break
                }

                min = max;
                max += quarter_size - 1;
            }

            break
        } else if current_position == FROM_POSITION {
            let quarter_size = spiral_circle;
            let distance_from_middle = quarter_size / 2;
            println!("{} + {} = {}", distance_from_middle, spiral_count, distance_from_middle + spiral_count);
            break
        }
    }
}