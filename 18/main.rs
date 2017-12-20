extern crate read_input;

use std::collections::HashMap;
use std::sync::mpsc;

mod program;
use program::get_register_or_value;

#[derive(Debug, PartialEq)]
enum Instruction {
    Set(String, String),
    Add(String, String),
    Mul(String, String),
    Mod(String, String),
    Snd(String),
    Rcv(String),
    Jump(String, String),
}

trait Program {
    fn set(&mut self, register: &String, value: &String);
    fn add(&mut self, register: &String, value: &String);
    fn mul(&mut self, register: &String, value: &String);
    fn modulous(&mut self, register: &String, value: &String);
    fn snd(&mut self, register: &String) -> Option<i64>;
    fn rcv(&mut self, register: &String, value: i64) -> bool;
    fn jump(&mut self, register: &String, value: &String);
    fn get_current_instruction(&self) -> i64;
}

struct ProgramPartOne {
    registers: HashMap<String, i64>,
    current_instruction: i64,
    frequency: i64,
}

impl Program for ProgramPartOne {
    fn set(&mut self, register: &String, value: &String) {
        self.current_instruction = program::set(&mut self.registers, register, value, self.current_instruction);
    }
    fn add(&mut self, register: &String, value: &String) {
        self.current_instruction = program::add(&mut self.registers, register, value, self.current_instruction);
    }
    fn mul(&mut self, register: &String, value: &String) {
        self.current_instruction = program::mul(&mut self.registers, register, value, self.current_instruction);
    }
    fn modulous(&mut self, register: &String, value: &String) {
        self.current_instruction = program::modulous(&mut self.registers, register, value, self.current_instruction);
    }
    fn snd(&mut self, register: &String) -> Option<i64> {
        self.frequency = get_register_or_value(&self.registers, register);
        self.current_instruction += 1;
        None
    }
    fn rcv(&mut self, register: &String, _: i64) -> bool {
        self.current_instruction += 1;
        if get_register_or_value(&self.registers, &register) != 0 {
            println!("{}", self.frequency);
            return true
        }
        false
    }
    fn jump(&mut self, register: &String, value: &String) {
        self.current_instruction = program::jump(&mut self.registers, register, value, self.current_instruction);
    }
    fn get_current_instruction(&self) -> i64 {
        self.current_instruction
    }
}

struct ProgramPartTwo {
    registers: HashMap<String, i64>,
    current_instruction: i64,
    waiting: bool,
}

impl Program for ProgramPartTwo {
    fn set(&mut self, register: &String, value: &String) {
        self.current_instruction = program::set(&mut self.registers, register, value, self.current_instruction);
    }
    fn add(&mut self, register: &String, value: &String) {
        self.current_instruction = program::add(&mut self.registers, register, value, self.current_instruction);
    }
    fn mul(&mut self, register: &String, value: &String) {
        self.current_instruction = program::mul(&mut self.registers, register, value, self.current_instruction);
    }
    fn modulous(&mut self, register: &String, value: &String) {
        self.current_instruction = program::modulous(&mut self.registers, register, value, self.current_instruction);
    }
    fn snd(&mut self, register: &String) -> Option<i64> {
        Some(get_register_or_value(&self.registers, register))
    }
    fn rcv(&mut self, register: &String, value: i64) -> bool {
        self.current_instruction += 1;
        self.registers.insert(register.clone(), value);
        false
    }
    fn jump(&mut self, register: &String, value: &String) {
        self.current_instruction = program::jump(&mut self.registers, register, value, self.current_instruction);
    }
    fn get_current_instruction(&self) -> i64 {
        self.current_instruction
    }
}

fn part_one(instructions: &Vec<Instruction>) {
    let mut program = ProgramPartOne{
        registers: HashMap::new(),
        current_instruction: 0,
        frequency: 0,
    };

    loop {
        let instruction = instructions.get(program.get_current_instruction() as usize).unwrap();
        match *instruction {
            Instruction::Set(ref register, ref value) => {
                program.set(&register, &value);
            },
            Instruction::Add(ref register, ref value) => {
                program.add(&register, &value);
            },
            Instruction::Mul(ref register, ref value) => {
                program.mul(&register, &value);
            },
            Instruction::Mod(ref register, ref value) => {
                program.modulous(&register, &value);
            },
            Instruction::Snd(ref register) => {
                program.snd(&register);
            },
            Instruction::Rcv(ref register) => {
                if program.rcv(&register, 0) {
                    break
                }
            },
            Instruction::Jump(ref condition, ref offset) => {
                program.jump(&condition, &offset);
            }
        }

        if program.get_current_instruction() < 0 || program.get_current_instruction() >= instructions.len() as i64 {
            break
        }
    }
}

fn main() {
    let text = match read_input::read_text("18/input.txt") {
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
            "add" => {
                instructions.push(Instruction::Add(iter.next().unwrap().to_string(), iter.next().unwrap().to_string()));
            },
            "mul" => {
                instructions.push(Instruction::Mul(iter.next().unwrap().to_string(), iter.next().unwrap().to_string()));
            },
            "mod" => {
                instructions.push(Instruction::Mod(iter.next().unwrap().to_string(), iter.next().unwrap().to_string()));
            },
            "snd" => {
                instructions.push(Instruction::Snd(iter.next().unwrap().to_string()));
            },
            "rcv" => {
                instructions.push(Instruction::Rcv(iter.next().unwrap().to_string()));
            },
            "jgz" => {
                instructions.push(Instruction::Jump(iter.next().unwrap().to_string(), iter.next().unwrap().to_string()));
            },
            _ => {
                panic!("Unsupported command: {:?}", line)
            }
        }
    }

    part_one(&instructions);
}
