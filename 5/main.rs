extern crate read_input;

fn solve(text: &String, part_two: bool) {
    let mut numbers: Vec<i32> = text.lines().map(|n| n.parse().unwrap() ).collect();
    let len = numbers.len() as i32;
    let mut current_index = 0i32;
    let mut steps = 0;

    loop {
        let value = {
            let value = numbers.get((current_index as usize)).unwrap();
            value.clone()
        };

        let next_index = current_index + value;

        let value = if !part_two || value < 3 {
            value + 1
        } else {
            value - 1
        };

        numbers[current_index as usize] = value;
        current_index = next_index;

        steps += 1;
        if next_index < 0 || next_index >= len {
            break
        }
    }

    println!("{}", steps);
}

fn main() {
    let text = match read_input::read_text("5/input.txt") {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };

    solve(&text, false);
    solve(&text, true);
}