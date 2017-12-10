extern crate read_input;
use std::collections::{HashSet, HashMap};

fn part_one(text: &String) -> String {
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

    let mut root_program: String = String::new();

    for program in all_programs {
        if !programs_supported.contains(&program) && !programs_at_end_of_tower.contains(&program) {
            root_program = program.clone();
            println!("{}", program);
        }
    }

    root_program
}

#[derive(Debug)]
struct Program {
    name: String,
    weight: usize,
    total_weight: usize,
    programs: Vec<String>,
}

fn apply_total_weight_to_program(programs: &mut HashMap<String, Program>, name: &String) -> usize {
    let new_weight = {
        let sub_programs = programs.get(name).unwrap().programs.clone();
        let sub_weights = sub_programs.iter().fold(0, |sum, n| sum + apply_total_weight_to_program(programs, n));
        programs.get(name).unwrap().weight + sub_weights
    };

    let program = programs.get_mut(name).unwrap();
    program.total_weight = new_weight;
    program.total_weight
}

fn sift_through_tree_to_find_unbalance(programs: &HashMap<String, Program>, name: &String, last_layer_uneven: bool) -> bool {
    let program = programs.get(name).unwrap();
    let mut weight = 0;
    let mut even_balance = true;
    for name in &program.programs {
        let sub_program = programs.get(name).unwrap();
        if weight == 0 {
            weight = sub_program.total_weight;
        } else if sub_program.total_weight != weight {
            even_balance = false;
        }
    }

    // if next layer balances out, that means this program is on the layer that is not balanced
    if even_balance && last_layer_uneven {
        return true
    }

    let mut current_program_bad = false;
    for name in &program.programs {
        let is_even = sift_through_tree_to_find_unbalance(programs, name, !even_balance);
        if is_even && last_layer_uneven {
            current_program_bad = true;
        }
    }

    if current_program_bad {
        let weights = program.programs.iter().map(|p| {
            let program = programs.get(p).unwrap();
            [program.weight, program.total_weight]
        }).collect::<Vec<[usize; 2]>>();
        println!("possibly bad sub programs for: {}", program.name);
        println!("{:?}", weights);
    }

    false
}

fn part_two(text: &String, root_program: &String) {
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

        programs.insert(program_name.clone(), Program{ name: program_name, weight: weight, programs: sub_programs, total_weight: 0 });
    }

    apply_total_weight_to_program(&mut programs, root_program);
    sift_through_tree_to_find_unbalance(&programs, root_program, false);
}

fn main() {
    let text = match read_input::read_text("7/input.txt") {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };

    let root_program = part_one(&text);
    part_two(&text, &root_program);
}