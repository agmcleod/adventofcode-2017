extern crate read_input;

use std::collections::HashMap;

fn build_maze_map(input: &String) -> ((i16, i16), HashMap<(i16, i16), char>) {
    let mut maze = HashMap::new();
    let mut start_coordinates = (0i16, 0i16);

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if y == 0 && ch == '|' {
                start_coordinates.0 = x as i16;
                start_coordinates.1 = y as i16;
                maze.insert((x as i16, y as i16), ch);
            } else if y != 0 && ch != ' ' {
                maze.insert((x as i16, y as i16), ch);
            }
        }
    }

    (start_coordinates, maze)
}

fn get_next_direction(maze: &HashMap<(i16, i16), char>, current_pos: &(i16, i16), direction: &(i16, i16)) -> Option<(i16, i16)> {
    if direction.0 != -1 && direction.0 != 1 && maze.contains_key(&(current_pos.0 - 1, current_pos.1)) {
        let value = maze.get(&(current_pos.0 - 1, current_pos.1)).unwrap();
        if *value != '|' {
            return Some((-1, 0))
        }
    } else if direction.0 != 1 && direction.0 != -1 && maze.contains_key(&(current_pos.0 + 1, current_pos.1)) {
        let value = maze.get(&(current_pos.0 + 1, current_pos.1)).unwrap();
        if *value != '|' {
            return Some((1, 0))
        }
    } else if direction.1 != -1 && direction.1 != 1 && maze.contains_key(&(current_pos.0, current_pos.1 - 1)) {
        let value = maze.get(&(current_pos.0, current_pos.1 - 1)).unwrap();
        if *value != '-' {
            return Some((0, -1))
        }
    } else if direction.1 != 1 && direction.1 != -1 && maze.contains_key(&(current_pos.0, current_pos.1 + 1)) {
        let value = maze.get(&(current_pos.0, current_pos.1 + 1)).unwrap();
        if *value != '-' {
            return Some((0, 1))
        }
    }

    None
}

fn main() {
    let text = match read_input::read_text("19/input.txt") {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };

    let (mut position, maze) = build_maze_map(&text);
    let mut direction = (0i16, 1i16);
    let mut letters: Vec<char> = Vec::new();
    let mut steps = 1;
    loop {
        if let Some(next_value) = maze.get(&(position.0 + direction.0, position.1 + direction.1)) {
            steps += 1;
            position.0 += direction.0;
            position.1 += direction.1;
            if *next_value != '-' && *next_value != '|' && *next_value != '+' {
                letters.push(*next_value);
            }
        } else {
            // just let loop handle next iteration from here, as it handles everything anyways
            if let Some(dir) = get_next_direction(&maze, &position, &direction) {
                direction = dir;
            } else {
                break
            }
        }
    }

    let letters: String = letters.iter().map(|c| c.to_string()).collect();
    println!("{} - {}", letters, steps);
}