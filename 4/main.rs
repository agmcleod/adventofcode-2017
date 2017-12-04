extern crate read_input;
use std::collections::HashSet;

fn part_one(text: &String) {
    let mut valid_count = 0;
    for line in text.lines() {
        let mut words: HashSet<String> = HashSet::new();

        let mut valid = true;
        for word in line.split(" ") {
            if words.contains(&word.to_string()) {
                valid = false;
            } else {
                words.insert(word.to_string());
            }
        }

        if valid {
            valid_count += 1;
        }
    }

    println!("{}", valid_count);
}

fn part_two(text: &String) {
    let mut valid_count = 0;
    for line in text.lines() {
        let mut words: HashSet<String> = HashSet::new();

        let mut valid = true;
        for word in line.split(" ") {
            let mut letters = word.split("").filter(|c| *c != "").map(|c| c.to_string()).collect::<Vec<String>>();
            letters.sort();
            let sorted_word: String = letters.into_iter().collect();
            if words.contains(&sorted_word) {
                valid = false;
            } else {
                words.insert(sorted_word);
            }
        }

        if valid {
            valid_count += 1;
        }
    }

    println!("{}", valid_count);
}

fn main() {
    let text = match read_input::read_text("4/input.txt") {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };

    part_one(&text);
    part_two(&text);
}