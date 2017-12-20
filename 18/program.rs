use std::collections::HashMap;

pub fn get_register_or_value(registers: &HashMap<String, i64>, register: &String) -> i64 {
    if let Some(register) = registers.get(register) {
        return *register
    }

    if let Ok(value) = register.parse() {
        return value
    }

    0
}

pub fn set(registers: &mut HashMap<String, i64>, register: &String, value: &String, current_instruction: i64) -> i64 {
    let value = get_register_or_value(&registers, value);
    registers.insert(register.clone(), value);
    current_instruction + 1
}

pub fn add(registers: &mut HashMap<String, i64>, register: &String, value: &String, current_instruction: i64) -> i64 {
    let current_value = get_register_or_value(&registers, register);
    let instruction_value = get_register_or_value(&registers, value);
    registers.insert(register.clone(), current_value + instruction_value);
    current_instruction + 1
}

pub fn mul(registers: &mut HashMap<String, i64>, register: &String, value: &String, current_instruction: i64) -> i64 {
    let current_value = get_register_or_value(&registers, register);
    let instruction_value = get_register_or_value(&registers, value);
    registers.insert(register.clone(), current_value * instruction_value);
    current_instruction + 1
}

pub fn modulous(registers: &mut HashMap<String, i64>, register: &String, value: &String, current_instruction: i64) -> i64 {
    let current_value = get_register_or_value(&registers, register);
    let instruction_value = get_register_or_value(&registers, value);
    registers.insert(register.clone(), current_value % instruction_value);
    current_instruction + 1
}

pub fn jump(registers: &mut HashMap<String, i64>, condition: &String, offset: &String, current_instruction: i64) -> i64 {
    if get_register_or_value(&registers, &condition) > 0 {
        current_instruction + get_register_or_value(&registers, &offset)
    } else {
        current_instruction + 1
    }
}
