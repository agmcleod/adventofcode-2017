enum Mode {
    A, B
}

fn get_new_value(value: usize, mode: Mode) -> usize {
    let new_value = match mode {
        Mode::A => value * 16807,
        Mode::B => value * 48271,
    };
    new_value % 2147483647
}

fn add_lead_zeroes(string: &mut String, desired_length: usize) {
    *string = format!("{text:0>width$}", width=desired_length, text=string);
}

fn part_one() {
    let mut a = 618;
    let mut b = 814;

    let mut judge_count = 0;

    for _ in 0..40_000_000 {
        a = get_new_value(a, Mode::A);
        b = get_new_value(b, Mode::B);

        let mut binary_value_a = format!("{:b}", a);
        let mut binary_value_b = format!("{:b}", b);

        add_lead_zeroes(&mut binary_value_a, 32);
        add_lead_zeroes(&mut binary_value_b, 32);

        if binary_value_a[binary_value_a.len()-16..binary_value_a.len()] == binary_value_b[binary_value_b.len()-16..binary_value_b.len()] {
            judge_count += 1;
        }
    }

    println!("{}", judge_count);
}

fn part_two() {
    let mut a = 618;
    let mut b = 814;

    let mut judge_count = 0;

    for _ in 0..5_000_000 {
        loop {
            a = get_new_value(a, Mode::A);
            if a % 4 == 0 {
                break
            }
        }
        loop {
            b = get_new_value(b, Mode::B);
            if b % 8 == 0 {
                break
            }
        }

        let mut binary_value_a = format!("{:b}", a);
        let mut binary_value_b = format!("{:b}", b);

        add_lead_zeroes(&mut binary_value_a, 32);
        add_lead_zeroes(&mut binary_value_b, 32);

        if binary_value_a[binary_value_a.len()-16..binary_value_a.len()] == binary_value_b[binary_value_b.len()-16..binary_value_b.len()] {
            judge_count += 1;
        }
    }

    println!("{}", judge_count);
}

fn main() {
    // part_one();
    part_two();
}