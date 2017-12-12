extern crate read_input;
use std::cmp;

fn move_by_direction(position: &mut (isize, isize, isize), direction: &String, steps_away: &mut isize) {
    match direction.as_ref() {
        "n" => {
            position.1 += 1;
            position.2 -= 1;
        },
        "s" => {
            position.1 -= 1;
            position.2 += 1;
        },
        "ne" => {
            position.0 += 1;
            position.2 -= 1;
        },
        "se" => {
            position.0 += 1;
            position.1 -= 1;
        },
        "sw" => {
            position.0 -= 1;
            position.2 += 1;
        },
        "nw" => {
            position.0 -= 1;
            position.1 += 1;
        },
        _ => {
            println!("unfound: \'{}\'", direction);
        }
    }

    *steps_away = cmp::max(*steps_away, calculate_steps_away(&position));
}

fn calculate_steps_away(position: &(isize, isize, isize)) -> isize {
    (position.0.abs() + position.1.abs() + position.2.abs()) / 2
}

fn main() {
    let text = match read_input::read_text("11/input.txt") {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };

    let directions: Vec<String> = text.trim().split(",").map(|s| s.to_string()).collect();
    let mut position = (0isize, 0isize, 0isize);
    let mut steps_away = 0;

    for direction in &directions {
        move_by_direction(&mut position, direction, &mut steps_away);
    }

    println!("{}", calculate_steps_away(&position));
    println!("{}", steps_away);
}