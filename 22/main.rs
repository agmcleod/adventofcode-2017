extern crate read_input;

use std::collections::HashMap;
use std::fmt;
use std::f32::consts::PI;

#[derive(PartialEq)]
enum NodeStatus {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl fmt::Debug for NodeStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match *self {
            NodeStatus::Clean => ".",
            NodeStatus::Weakened => "W",
            NodeStatus::Infected => "#",
            NodeStatus::Flagged => "F",
        };
        write!(f, "{}", value)
    }
}

fn set_next_direction(status: &NodeStatus, direction: &mut (i32, i32)) {
    let angle = match *status {
        NodeStatus::Clean => {
            -90.0 / 180.0 * PI
        },
        NodeStatus::Weakened => return,
        NodeStatus::Infected => {
            90.0 / 180.0 * PI
        },
        NodeStatus::Flagged => {
            1.0 * PI
        },
    };

    let x = direction.0 as f32;
    let y = direction.1 as f32;
    direction.0 = (x * angle.cos() - y * angle.sin()).round() as i32;
    direction.1 = (x * angle.sin() + y * angle.cos()).round() as i32;
}

fn set_next_node_state(status: &mut NodeStatus, part_one: bool) {
    if part_one {
        match *status {
            NodeStatus::Clean => *status = NodeStatus::Infected,
            NodeStatus::Infected => *status = NodeStatus::Clean,
            _ => {},
        }
    } else {
        match *status {
            NodeStatus::Clean => *status = NodeStatus::Weakened,
            NodeStatus::Weakened => *status = NodeStatus::Infected,
            NodeStatus::Infected => *status = NodeStatus::Flagged,
            NodeStatus::Flagged => *status = NodeStatus::Clean,
        }
    }
}

fn build_grid(text: &String) -> (HashMap<(i32, i32), NodeStatus>, i32) {
    let mut grid = HashMap::new();

    let mut line_count = 0;
    for (y, line) in text.lines().enumerate() {
        line_count += 1;
        for (x, value) in line.trim().chars().enumerate() {
            grid.insert((x as i32, y as i32), match value {
                '#' => NodeStatus::Infected,
                '.' => NodeStatus::Clean,
                _ => panic!("Invalid value from input: {}", value),
            });
        }
    }

    line_count /= 2;

    (grid, line_count)
}

fn simulate_virus(text: &String, iterations: usize, part_one: bool) {
    let (mut grid, half_size) = build_grid(text);

    let mut direction = (0i32, -1i32);
    let mut position = (half_size, half_size);

    let mut infection_bursts = 0;
    for _ in 0..iterations {
        if !grid.contains_key(&position) {
            grid.insert(position.clone(), NodeStatus::Clean);
        }

        let mut node = grid.get_mut(&position).unwrap();
        set_next_direction(node, &mut direction);
        set_next_node_state(&mut node, part_one);
        if *node == NodeStatus::Infected {
            infection_bursts += 1;
        }

        position.0 += direction.0;
        position.1 += direction.1;
    }

    println!("{}", infection_bursts);
}

fn main() {
    let text = match read_input::read_text("22/input.txt") {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };

    // simulate_virus(&text, 10000, true);
    simulate_virus(&text, 10_000_000, false);
}