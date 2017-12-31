enum State {
    A,
    B,
    C,
    D,
    E,
    F,
}

fn process_turing_step(state: &mut State, tape: &mut Vec<usize>, position: &mut usize) {
    let mut pos = *position as i64;
    match *state {
        State::A => {
            if tape[*position] == 0 {
                tape[*position] = 1;
                pos += 1;
                *state = State::B;
            } else {
                tape[*position] = 0;
                pos -= 1;
                *state = State::C;
            }
        },
        State::B => {
            tape[*position] = 1;
            pos -= 1;
            if tape[*position] == 0 {
                *state = State::A;
            } else {
                *state = State::D;
            }
        },
        State::C => {
            if tape[*position] == 0 {
                tape[*position] = 1;
                pos += 1;
                *state = State::D;
            } else {
                tape[*position] = 0;
                pos += 1;
                *state = State::C;
            }
        },
        State::D => {
            if tape[*position] == 0 {
                pos -= 1;
                *state = State::B;
            } else {
                tape[*position] = 0;
                pos += 1;
                *state = State::E;
            }
        },
        State::E => {
            if tape[*position] == 0 {
                tape[*position] = 1;
                pos += 1;
                *state = State::C;
            } else {
                pos -= 1;
                *state = State::F;
            }
        },
        State::F => {
            if tape[*position] == 0 {
                tape[*position] = 1;
                pos -= 1;
                *state = State::E;
            } else {
                pos += 1;
                *state = State::A;
            }
        },
    }

    pad_zeroes_for_out_of_bounds(tape, pos);
}

fn pad_zeroes_for_out_of_bounds(tape: &mut Vec<usize>, pos: i64) {
    if pos < 0 {
        for _ in 0..pos.abs() {
            tape.insert(0, 0);
        }
    } else if pos as usize >= tape.len() {
        for _ in 0..(pos.abs() as usize - (tape.len() - 1)) {
            tape.push(0);
        }
    }
}

fn main() {
    let mut tape = vec![0];
    let mut state = State::A;
    let mut position = 0;

    for _ in 0..12656374 {
        process_turing_step(&mut state, &mut tape, &mut position);
    }

    println!("{}", tape.iter().fold(0, |sum, v| sum + v));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_adding_zeroes_index_in_middle() {
        let mut tape = vec![0, 0, 0];
        pad_zeroes_for_out_of_bounds(&mut tape, 1);
        assert_eq!(tape.len(), 3);
    }

    #[test]
    fn test_adding_zeroes_index_before_front() {
        let mut tape = vec![0, 0, 0];
        pad_zeroes_for_out_of_bounds(&mut tape, -1);
        assert_eq!(tape.len(), 4);
    }

    #[test]
    fn test_adding_zeroes_index_after_end() {
        let mut tape = vec![0, 0, 0];
        pad_zeroes_for_out_of_bounds(&mut tape, 3);
        assert_eq!(tape.len(), 4);
    }
}
