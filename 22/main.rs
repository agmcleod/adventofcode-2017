extern crate read_input;

fn main() {
    let text = match read_input::read_text("21/input.txt") {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };
}