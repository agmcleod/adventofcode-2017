use std::collections::HashMap;

enum State {
    A,
    B,
    C,
    D,
    E,
    F,
}

fn cursor_is_zero(tape: &HashMap<i64, usize>, position: &i64) -> bool {
    !tape.contains_key(position) || tape[position] == 0
}

fn process_turing_step(state: &mut State, tape: &mut HashMap<i64, usize>, position: &mut i64) {
    match *state {
        State::A => {
            if cursor_is_zero(tape, position) {
                tape.insert(*position, 1);
                *position += 1;
                *state = State::B;
            } else {
                tape.insert(*position, 0);
                *position -= 1;
                *state = State::C;
            }
        },
        State::B => {
            if cursor_is_zero(tape, position) {
                tape.insert(*position, 1);
                *position -= 1;
                *state = State::A;
            } else {
                tape.insert(*position, 1);
                *position -= 1;
                *state = State::D;
            }
        },
        State::C => {
            if cursor_is_zero(tape, position) {
                tape.insert(*position, 1);
                *position += 1;
                *state = State::D;
            } else {
                tape.insert(*position, 0);
                *position += 1;
                *state = State::C;
            }
        },
        State::D => {
            if cursor_is_zero(tape, position) {
                *position -= 1;
                *state = State::B;
            } else {
                tape.insert(*position, 0);
                *position += 1;
                *state = State::E;
            }
        },
        State::E => {
            if cursor_is_zero(tape, position) {
                tape.insert(*position, 1);
                *position += 1;
                *state = State::C;
            } else {
                *position -= 1;
                *state = State::F;
            }
        },
        State::F => {
            if cursor_is_zero(tape, position) {
                tape.insert(*position, 1);
                *position -= 1;
                *state = State::E;
            } else {
                *position += 1;
                *state = State::A;
            }
        },
    }
}

fn main() {
    let mut tape = HashMap::new();
    tape.insert(0i64, 0);
    let mut state = State::A;
    let mut position = 0;

    for _ in 0..12656374 {
        process_turing_step(&mut state, &mut tape, &mut position);
    }

    println!("{}", tape.iter().fold(0, |sum, v| sum + v.1));
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
