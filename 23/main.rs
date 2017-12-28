extern crate read_input;

use std::collections::HashMap;

mod program;
use program::get_register_or_value;

type Registers = HashMap<String, i64>;

#[derive(Debug, PartialEq)]
enum Instruction {
    Set(String, String),
    Mul(String, String),
    Sub(String, String),
    Jump(String, String),
}

trait ProgramBehaviour {
    fn set(&mut self, register: &String, value: &String);
    fn sub(&mut self, register: &String, value: &String);
    fn mul(&mut self, register: &String, value: &String);
    fn jump(&mut self, register: &String, value: &String);
    fn get_current_instruction(&self) -> i64;
}

struct Program {
    registers: Registers,
    current_instruction: i64,
}

impl ProgramBehaviour for Program {
    fn set(&mut self, register: &String, value: &String) {
        self.current_instruction = program::set(&mut self.registers, register, value, self.current_instruction);
    }
    fn sub(&mut self, register: &String, value: &String) {
        self.current_instruction = program::sub(&mut self.registers, register, value, self.current_instruction);
    }
    fn mul(&mut self, register: &String, value: &String) {
        self.current_instruction = program::mul(&mut self.registers, register, value, self.current_instruction);
    }
    fn jump(&mut self, register: &String, value: &String) {
        self.current_instruction = program::jump(&mut self.registers, register, value, self.current_instruction);
    }
    fn get_current_instruction(&self) -> i64 {
        self.current_instruction
    }
}

fn process(instructions: &Vec<Instruction>, part_two: bool) {
    let mut program = Program{
        registers: HashMap::new(),
        current_instruction: 0,
    };

    if part_two {
        program.registers.insert("a".to_string(), 1);
    }

    let mut multiply_counts = 0;
    let mut h_register = 0i64;
    let mut negative_jumps: HashMap<usize, Registers> = HashMap::new();

    loop {
        let instruction = instructions.get(program.get_current_instruction() as usize).unwrap();
        match *instruction {
            Instruction::Set(ref register, ref value) => {
                program.set(&register, &value);
            },
            Instruction::Sub(ref register, ref value) => {
                program.sub(&register, &value);
            },
            Instruction::Mul(ref register, ref value) => {
                program.mul(&register, &value);
                multiply_counts += 1;
            },
            Instruction::Jump(ref condition, ref offset) => {
                program.jump(&condition, &offset);
            }
        }

        let h = get_register_or_value(&program.registers, &"h".to_string());
        if part_two && h != h_register {
            h_register = h;
            println!("{}", h_register);
        }

        if program.get_current_instruction() < 0 || program.get_current_instruction() >= instructions.len() as i64 {
            break
        }
    }

    println!("multiply counts: {}", multiply_counts);
    println!("value in h: {:?}", program.registers.get(&"h".to_string()));
}

fn main() {
    let text = match read_input::read_text("23/input.txt") {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };

    let mut instructions: Vec<Instruction> = Vec::new();
    for line in text.lines() {
        let mut iter = line.split(" ");
        match iter.next().unwrap() {
            "set" => {
                instructions.push(Instruction::Set(iter.next().unwrap().to_string(), iter.next().unwrap().to_string()));
            },
            "mul" => {
                instructions.push(Instruction::Mul(iter.next().unwrap().to_string(), iter.next().unwrap().to_string()));
            },
            "sub" => {
                instructions.push(Instruction::Sub(iter.next().unwrap().to_string(), iter.next().unwrap().to_string()));
            },
            "jnz" => {
                instructions.push(Instruction::Jump(iter.next().unwrap().to_string(), iter.next().unwrap().to_string()));
            },
            _ => {
                panic!("Unsupported command: {:?}", line)
            }
        }
    }

    process(&instructions, false);
    // process(&instructions, true);
}
