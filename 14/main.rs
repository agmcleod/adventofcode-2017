extern crate knot_hash;
extern crate bytecount;
use std::collections::{HashMap, BinaryHeap};

fn find_next_root(grid: &HashMap<(usize, usize), usize>) -> Option<(usize, usize)> {
    for (coords, region) in grid {
        if *region == 0 {
            return Some(*coords)
        }
    }

    None
}

fn get_neighbours(coord: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::new();
    neighbours.push((coord.0 + 1, coord.1));
    neighbours.push((coord.0, coord.1 + 1));

    if (coord.0 > 0) {
        neighbours.push((coord.0 - 1, coord.1));
    }
    if (coord.1 > 0) {
        neighbours.push((coord.0, coord.1 - 1));
    }

    neighbours
}

fn get_region_count(grid: &mut HashMap<(usize, usize), usize>) -> usize {
    let mut group_id = 1;
    loop {
        if let Some(root) = find_next_root(&grid) {
            grid.insert(root, group_id);
            let mut nodes_to_traverse: BinaryHeap<(usize, usize)> = BinaryHeap::new();
            nodes_to_traverse.push(root);

            while let Some(node) = nodes_to_traverse.pop() {
                let neighbours = get_neighbours(&node);
                for neighbour in &neighbours {
                    if grid.contains_key(neighbour) {
                        if *grid.get(&neighbour).unwrap() != 0 {
                            continue
                        }
                        grid.insert(*neighbour, group_id);
                        nodes_to_traverse.push(*neighbour);
                    }
                }
            }

            group_id += 1;

        } else {
            break
        }
    }

    group_id - 1
}

fn main() {
    let key = "hxtvlmkl";

    let mut used_count = 0;
    let mut grid: HashMap<(usize, usize), usize> = HashMap::new();
    for n in 0..128 {
        let hash = knot_hash::create(&format!("{}-{}", key, n), 256);
        let value: String = hash.iter().map(|n| {
            let value = format!("{:b}", n);
            let mut resulting_value = value.clone();
            for _ in 0..(8 - value.len()) {
                resulting_value = format!("{}{}", 0, resulting_value);
            }
            resulting_value
        }).collect();

        for (i, ch) in value.chars().enumerate() {
            if ch == '1' {
                grid.insert((i, n), 0);
            }
        }

        used_count += bytecount::count(value.as_bytes(), b'1');
    }

    println!("used count: {}", used_count);
    println!("region count: {}", get_region_count(&mut grid));
}