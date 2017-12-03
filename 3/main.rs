use std::cmp;
use std::collections::HashMap;

const FROM_POSITION: usize = 289326;

fn size_of_spiral_for_iteration(iteration: usize) -> usize {
    (iteration * 4 - 4)
}

fn part_one_mathy() {
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
                    println!("{} + {} = {}", distance_from_middle, spiral_count, distance_from_middle + spiral_count);
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

/**
 * While i was able to figure out the math for the first one, lets try to walk the spiral
 *
 * Direction will say x & y direction of which to move next. I will need to count when to change direction.
 */
fn part_two_walk() {
    // start with a move right
    let mut direction: (isize, isize) = (1, 0);
    let mut position: (isize, isize) = (0, 0);
    let mut spiral: HashMap<(isize, isize), isize> = HashMap::new();
    spiral.insert((0, 0), 1);
    let mut direction_usage = 0;
    let mut line_length = 1;
    let mut line_walk_count = 0;

    loop {
        position.0 += direction.0;
        position.1 += direction.1;

        let sum = get_sum_of_adjacents(&spiral, &position);
        if sum > FROM_POSITION as isize {
            println!("P2 answer: {}", sum);
            break
        }
        spiral.insert(position.clone(), sum);
        line_walk_count += 1;

        if line_walk_count == line_length {
            direction_usage += 1;
            line_walk_count = 0;
            next_direction(&mut direction);
        }

        if direction_usage == 2 {
            line_length += 1;
            direction_usage = 0;
        }
    }
}

/**
 * This updates direction based on pattern discovered. Can see what is the next pattern as you manually write out steps to go around the spiral
 */
fn next_direction(direction: &mut (isize, isize)) {
    if *direction == (1, 0) {
        *direction = (0, -1);
    } else if *direction == (0, -1) {
        *direction = (-1, 0);
    } else if *direction == (-1, 0) {
        *direction = (0, 1);
    } else if *direction == (0, 1) {
        *direction = (1, 0);
    } else {
        panic!("Direction not found: {:?}", direction);
    }
}

/**
 * Gets the sum of the values equal to each adjacent value. Adjancent can be useful up to 8 directions
 */
fn get_sum_of_adjacents(spiral: &HashMap<(isize, isize), isize>, current_pos: &(isize, isize)) -> isize {
    let mut sum = 0isize;
    for x_offset in -1..2 {
        for y_offset in -1..2 {
            if x_offset == 0 && y_offset == 0 {
                continue
            }

            let mut pos = current_pos.clone();
            pos.0 += x_offset;
            pos.1 += y_offset;

            if let Some(value) = spiral.get(&(pos)) {
                sum += *value;
            }
        }
    }

    sum
}

fn main() {
    part_one_mathy();
    part_two_walk();
}