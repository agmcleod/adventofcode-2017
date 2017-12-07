extern crate read_input;
use std::collections::HashSet;

fn part_one(text: &String) {
    let mut all_programs: Vec<String> = Vec::new();
    let mut programs_supported: HashSet<String> = HashSet::new();
    let mut programs_at_top: HashSet<String> = HashSet::new();
    for line in text.lines() {
        let mut words_iter = line.split(" ");
        let pr;
        if let Some(program) = words_iter.next() {
            all_programs.push(program.to_string());
            pr = program.to_string();
        } else {
            continue
        }

        // skip weights & arrow if its there
        let mut words_iter = words_iter.skip(2);

        let mut count = 0;
        while let Some(word) = words_iter.next() {
            let word = word.replace(",", "");
            programs_supported.insert(word.to_string());
            count += 1;
        }

        if count == 0 {
            programs_at_top.insert(pr);
        }
    }

    for program in all_programs {
        if !programs_supported.contains(&program) && !programs_at_top.contains(&program) {
            println!("{}", program);
        }
    }
}

fn main() {
    let text = match read_input::read_text("7/input.txt") {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };

    part_one(&text);
}