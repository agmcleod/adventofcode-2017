extern crate read_input;

use std::collections::HashMap;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, Barrier};
use std::time::Duration;

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
        self.current_instruction += 1;
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

fn part_two(instructions: Arc<Vec<Instruction>>) {
    let mut handles = Vec::with_capacity(2);
    let barrier = Arc::new(Barrier::new(2));

    let (sender, receiver) = channel::<i64>();
    let (sender2, receiver2) = channel::<i64>();
    let c = barrier.clone();
    let p0_instructions = instructions.clone();
    handles.push(thread::spawn(move|| {
        let mut p0 = ProgramPartTwo{
            registers: HashMap::new(),
            current_instruction: 0,
        };
        p0.registers.insert("p".to_string(), 0);
        start_loop(p0_instructions, &mut p0, "Program 0", sender2, receiver);
        c.wait();
    }));

    let c = barrier.clone();
    let p1_instructions = instructions.clone();
    handles.push(thread::spawn(move|| {
        let mut p1 = ProgramPartTwo{
            registers: HashMap::new(),
            current_instruction: 0,
        };
        p1.registers.insert("p".to_string(), 1);
        start_loop(p1_instructions, &mut p1, "Program 1", sender, receiver2);
        c.wait();
    }));

    for handle in handles {
        handle.join().unwrap();
    }
}

fn start_loop(instructions: Arc<Vec<Instruction>>, program: &mut Program, name: &str, sender: Sender<i64>, receiver: Receiver<i64>) {
    let mut send_count = 0;
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
                if let Some(value) = program.snd(&register) {
                    sender.send(value).unwrap();
                    send_count += 1;
                }
            },
            Instruction::Rcv(ref register) => {
                match receiver.recv_timeout(Duration::new(5, 0)) {
                    Ok(value) => {
                        program.rcv(&register, value)
                    },
                    Err(_) => {
                        println!("Timed out with: {}", name);
                        break
                    },
                };
            },
            Instruction::Jump(ref condition, ref offset) => {
                program.jump(&condition, &offset);
            }
        }

        if program.get_current_instruction() < 0 || program.get_current_instruction() >= instructions.len() as i64 {
            break
        }
    }

    println!("{} send count: {}", name, send_count);
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
    part_two(Arc::new(instructions));
}
