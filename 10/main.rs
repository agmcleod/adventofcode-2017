extern crate knot_hash;

const LENGTH: usize = 256;

fn main() {
    let lengths = [129,154,49,198,200,133,97,254,41,6,2,1,255,0,191,108];

    let mut numbers = knot_hash::build_numbers(LENGTH);
    knot_hash::run_iteration(&mut numbers, LENGTH, &lengths, &mut 0, &mut 0);
    println!("{}", numbers[0] * numbers[1]);

    let lengths: Vec<String> = lengths.iter().map(|n| format!("{}", n)).collect();
    let values = knot_hash::create(&lengths.join(","), LENGTH);
    println!("{}", knot_hash::numbers_to_hash(&values));
}