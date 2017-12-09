extern crate read_input;
use std::collections::{HashSet, HashMap};

fn part_one(text: &String) {
    let mut all_programs: Vec<String> = Vec::new();
    let mut programs_supported: HashSet<String> = HashSet::new();
    let mut programs_at_end_of_tower: HashSet<String> = HashSet::new();
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
            programs_at_end_of_tower.insert(pr);
        }
    }

    for program in all_programs {
        if !programs_supported.contains(&program) && !programs_at_end_of_tower.contains(&program) {
            println!("{}", program);
        }
    }
}

#[derive(Debug)]
struct Program {
    name: String,
    weight: usize,
    programs: Vec<String>,
}

fn part_two(text: &String) {
    let mut programs: HashMap<String, Program> = HashMap::new();
    for line in text.lines() {
        let mut words_iter = line.split(" ");
        let program_name = words_iter.next().unwrap().to_string();
        let weight = words_iter.next().unwrap().to_string();
        let weight: usize = weight.replace("(", "").replace(")", "").parse().unwrap();

        let mut sub_programs: Vec<String> = Vec::new();
        if let Some(_) = words_iter.next() {
            while let Some(sub_program) = words_iter.next() {
                sub_programs.push(sub_program.to_string().replace(",", ""));
            }
        }

        programs.insert(program_name.clone(), Program{ name: program_name, weight: weight, programs: sub_programs });
    }

    for (key, program) in programs {
        println!("{:?}", program);
    }
}

fn main() {
    let text = match read_input::read_text("7/input.txt") {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };

    part_one(&text);
    part_two(&text);
}