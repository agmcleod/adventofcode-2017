extern crate read_input;
use std::str::Split;
use std::collections::HashMap;

enum Command {
    Spin(usize),
    Exchange(usize, usize),
    Partner(String, String),
}

fn next_chunk_as_number(pieces: &mut Split<&str>, name: &str) -> usize {
    if let Some(piece) = pieces.next() {
        match piece.trim().parse() {
            Ok(n) => n,
            Err(e) => panic!("{:?} - could not parse number for exchange: {} - {:?}", e, name, pieces.collect::<String>()),
        }
    } else {
        panic!("No string for exchange: {}", name);
    }
}

fn next_chunk_as_string(pieces: &mut Split<&str>, name: &str) -> String {
    if let Some(piece) = pieces.next() {
        piece.to_string()
    } else {
        panic!("No string for partner: {}", name);
    }
}

fn part_one(commands: &Vec<Command>, dancers: &mut [&str; 16]) {
    for command in commands.iter() {
        match *command {
            Command::Spin(amount) => {
                let mut copy_dancers = dancers.clone();
                let len = dancers.len();

                for i in 0..len {
                    copy_dancers[(amount + i) % len] = dancers[i];
                }

                *dancers = copy_dancers;
            },
            Command::Exchange(ref from, ref to) => {
                dancers.swap(*from, *to);
            },
            Command::Partner(ref from, ref to) => {
                let from = dancers.iter().position(|v| *v == from).unwrap();
                let to = dancers.iter().position(|v| *v == to).unwrap();
                dancers.swap(from, to);
            }
        }
    }
}

fn main() {
    let text = match read_input::read_text("16/input.txt") {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };

    let mut commands: Vec<Command> = Vec::new();

    for instruction in text.split(",") {
        let mut chars = instruction.chars();
        let key = chars.next().unwrap();

        /**
         * Trying an enum value setup. Could just apply the parsed values right away, but figure i might as well get some practice here.
         */
        let command = match key {
            's' => {
                let num: String = chars.take(instruction.len() - 1).collect();
                match num.parse() {
                    Ok(n) => Command::Spin(n),
                    Err(e) => panic!("{:?} - could not parse number for spin", e),
                }
            },
            'x' => {
                let positions: String = chars.take(instruction.len() - 1).collect();
                let mut pieces = positions.split("/");
                let from = next_chunk_as_number(&mut pieces, "from");
                let to = next_chunk_as_number(&mut pieces, "to");

                Command::Exchange(from, to)
            },
            'p' => {
                let partners: String = chars.take(instruction.len() - 1).collect();
                let mut pieces = partners.split("/");
                let from = next_chunk_as_string(&mut pieces, "from");
                let to = next_chunk_as_string(&mut pieces, "to");

                Command::Partner(from, to)
            },
            _ => panic!("unfound value: {}", key),
        };

        commands.push(command);
    }

    let mut dancers = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p"];
    let mut p2_dancers = dancers.clone();
    part_one(&commands, &mut dancers);
    println!("{}", dancers.join(""));

    let mut tracker: HashMap<String, usize> = HashMap::new();
    let mut dance_poses: Vec<String> = Vec::new();
    for i in 0..1_000_000_000 {
        part_one(&commands, &mut p2_dancers);
        let key = p2_dancers.join("");
        if let Some(repeat) = tracker.get(&key) {
            println!("{}", dance_poses[(repeat + 1_000_000_000 - i - 1) % dance_poses.len()]);
            break
        }
        dance_poses.push(key.clone());
        tracker.insert(key, dance_poses.len() - 1);
    }
}