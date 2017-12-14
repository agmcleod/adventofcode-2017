extern crate read_input;
use std::collections::HashMap;

struct Depth {
    depth: usize,
    list: Vec<usize>,
}

fn main() {
    let text = match read_input::read_text("13/input.txt") {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };

    let mut layers: HashMap<usize, Depth> = HashMap::new();
    let mut last_layer: usize = 0;

    for line in text.lines() {
        let mut it = line.split(": ");
        let key = it.next().unwrap().parse().unwrap();
        let depth = it.next().unwrap().parse().unwrap();
        last_layer = key;

        let size = depth * 2 - 2;
        let mut list: Vec<usize> = Vec::with_capacity(size);
        for n in 0..size {
            if n >= depth {
                list.push(depth - 2 - (n - depth));
            } else {
                list.push(n);
            }
        }

        layers.insert(key, Depth{ depth: depth, list: list });
    }

    let mut severity = 0;
    for n in 0..(last_layer+1) {
        if let Some(depth) = layers.get(&n) {
            if depth.list[n % depth.list.len()] == 0 {
                severity += n * depth.depth;
            }
        }
    }

    println!("{}", severity);
}