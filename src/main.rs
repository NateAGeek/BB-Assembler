extern crate termion;

use std::env;
use std::io;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

const POSITIVE_NUMBER: bool = true;
const NEGATIVE_NUMBER: bool = true;

struct Container {
    name: String
}

struct ScaleValue {
    sign: bool,
    bit_length: u64,
    array_length: u64,
    value: [u64; 1],
    value_type: String
}

struct Register {
    container: Container,
    value: u64
}

struct Variable {
    container: Container,
    value: ScaleValue
}

impl ScaleValue {
    fn new() -> ScaleValue {
        ScaleValue {
            sign: POSITIVE_NUMBER,
            bit_length: 64,
            array_length: 1,
            value: [0; 1],
            value_type: "number".to_string()
        }
    }

//    fn increaseSize() {
//        old_val = self.value;
//        value = [u64; array_length*2];
//        array_length *= 2;
//    }
//    fn getBitValue(index: u64) {
//        let value_chunk_index = index/64;
//        let value_index = index%64;
//        return (value[value_chunk_index] >> value_index) & 1u64;
//    }
}

fn asm_mov(loc_one: &String, loc_two: &String, registers: &mut HashMap<String, u64>) {
    *registers.get_mut(loc_two).unwrap() = *registers.get::<String>(loc_one).unwrap();

    println!("MOV from({}) to({})", loc_one, loc_two);
    println!("key_value {}", registers.get(loc_one).unwrap());
    println!("key_value {}", registers.get(loc_two).unwrap());
}

fn asm_add(loc_one: &String, loc_two: &String, registers: &mut HashMap<String, u64>) {
    *registers.get_mut(loc_two).unwrap() += *registers.get::<String>(loc_one).unwrap();

    println!("ADD from({}) to({})", loc_one, loc_two);
    println!("key_value {}", registers.get(loc_one).unwrap());
    println!("key_value {}", registers.get(loc_two).unwrap());
}

fn asm_sub(loc_one: &String, loc_two: &String, registers: &mut HashMap<String, u64>) {
    *registers.get_mut(loc_two).unwrap() -= *registers.get::<String>(loc_one).unwrap();

    println!("SUB from({}) to({})", loc_one, loc_two);
    println!("key_value {}", registers.get(loc_one).unwrap());
    println!("key_value {}", registers.get(loc_two).unwrap());
}

fn asm_mul(loc_one: &String, loc_two: &String, registers: &mut HashMap<String, u64>) {
    *registers.get_mut(loc_two).unwrap() *= *registers.get::<String>(loc_one).unwrap();

    println!("SUB from({}) to({})", loc_one, loc_two);
    println!("key_value {}", registers.get(loc_one).unwrap());
    println!("key_value {}", registers.get(loc_two).unwrap());
}

fn is_supported(command: &String, available_commands: &Vec<String>) -> bool {
    return true;
}

fn create_registers(registers: &mut HashMap<String, u64>) -> (){
    registers.entry("a".to_string()).or_insert(0);
    registers.entry("b".to_string()).or_insert(100);
}

fn start_interpreter(available_registers: Vec<String>, available_commands: Vec<String>) -> Result<(), io::Error> {

    let mut string_variables: HashMap<String, String> = HashMap::new();
    let mut number_variables: HashMap<String, u64> = HashMap::new();
    let mut registers: HashMap<String, u64> = HashMap::new();

    let mut file_name: String = env::args().nth(1).unwrap();
    let mut file = try!(File::open(file_name));
    let mut file_text = String::new();

    create_registers(&mut registers);

    try!(file.read_to_string(&mut file_text));
    for line in file_text.lines() {
        let mut line_splits = line.split_whitespace();
        let mut command = line_splits.next().unwrap().to_string();
        if is_supported(&command, &available_commands) {
            let mut loc_1 = line_splits.next().unwrap().replace(",", "").to_string();
            let mut loc_2 = line_splits.next().unwrap().to_string();
            if command == "mov" {
                asm_mov(&mut loc_1, &mut loc_2, &mut registers);
            } else if command == "add" {
                asm_add(&mut loc_1, &mut loc_2, &mut registers);
            } else if command == "sub" {
                asm_sub(&mut loc_1, &mut loc_2, &mut registers);
            } else if command == "mul" {
                asm_mul(&mut loc_1, &mut loc_2, &mut registers);
            } else if command == "div" {
            
            }
            println!("a = {}", registers.get(&"a".to_string()).unwrap());
            println!("b = {}", registers.get(&"b".to_string()).unwrap());
        }
        println!("Line: {}", line);
        println!("Command {}", command);
    }
    Ok(())
}

fn generate_registers(regs: Vec<String>) -> Vec<String> {
    let mut extensions = Vec::new();
            extensions.push("".to_string());
            extensions.push("b".to_string());
            extensions.push("n".to_string());
            extensions.push("w".to_string());
            extensions.push("d".to_string());
            extensions.push("q".to_string());
                              
    let mut extended_registers = Vec::new();

    for reg in regs.iter() {
        for exe in extensions.iter() {
            extended_registers.push(format!("{}{}", reg, exe));
        }
    }

    return extended_registers;
}

fn test_asm() {
    let mut registers:Vec<String> = Vec::new();
            registers.push("a".to_string());
            registers.push("b".to_string());
            registers.push("c".to_string());
            registers.push("d".to_string());
            registers.push("e".to_string());
            registers.push("f".to_string());
            registers.push("g".to_string());
            registers.push("h".to_string());
            registers.push("i".to_string());
            registers.push("j".to_string());
            registers.push("k".to_string());
            registers.push("l".to_string());
            registers.push("m".to_string());
            registers.push("n".to_string());
            registers.push("o".to_string());
            registers.push("p".to_string());

    let mut available_commands = Vec::new();
            available_commands.push("mov".to_string());
            available_commands.push("add".to_string());
            available_commands.push("sub".to_string());

    let mut available_registers = generate_registers(registers);

    start_interpreter(available_registers, available_commands);
}

fn test_things () {
    let taco = ScaleValue::new();
    println!("sign: {}", taco.sign);    
}

fn main() {
    //test_asm();
    test_things();
}
