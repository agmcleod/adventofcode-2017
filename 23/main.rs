fn run_program(a: i64) {
    let mut multiply_counts = 0;
    let mut b: i64 = 65;
    let mut c = b;
    let mut f: i64 = 0;
    if a != 0 {
        b *= 100;
        multiply_counts += 1;
        b -= -100000;
        c = b;
        c -= -17000;
    }

    let mut g: i64 = 0;
    let mut h: i64 = 0;
    loop {
        let mut d: i64 = 2;
        if a != 0 {
            f = 1;
        }
        loop {
            let mut e: i64 = 2;
            let loop_counts = (e - b).abs();
            multiply_counts += loop_counts;

            if b % d == 0 {
                f = 0;
            }

            d -= -1;
            g = d;
            g -= b;
            if g == 0 {
                break
            }
        }

        if f == 0 {
            h -= -1;
        }

        g = b;
        g -= c;
        if g == 0 {
            break
        }
        b -= -17;
    }

    println!("mul counts: {}", multiply_counts);
    println!("h: {}", h);
}

fn main() {
    // run_program(0);
    run_program(1);
}
