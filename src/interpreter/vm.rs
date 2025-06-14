// have three vecs, vec 1 is the stack, vec 2 is the global vars, vec 3 is the local vars

use std::{collections::HashMap, process};

use super::constants;

#[derive(Debug)]
pub enum Opcode {
    PushInt,
    PushStr,
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,
    Eq,
    Neq,
    Lt,
    Gt,
    Jmp,
    JmpIfTrue,
    JmpIfFalse,
    Call,
    Ret,
    LoadLocal,
    StoreLocal,
    Load,
    Store,
    Halt,
}

impl Opcode {
    fn from_u8(byte: u8) -> Option<Self> {
        match byte {
            0x01 => Some(Opcode::PushInt),
            0x02 => Some(Opcode::PushStr),
            0x03 => Some(Opcode::Pop),
            0x04 => Some(Opcode::Add),
            0x05 => Some(Opcode::Sub),
            0x06 => Some(Opcode::Mul),
            0x07 => Some(Opcode::Div),
            0x15 => Some(Opcode::Mod),
            0x17 => Some(Opcode::Neg),
            0x08 => Some(Opcode::Eq),
            0x09 => Some(Opcode::Neq),
            0x0a => Some(Opcode::Lt),
            0x0b => Some(Opcode::Gt),
            0x0c => Some(Opcode::Jmp),
            0x0d => Some(Opcode::JmpIfTrue),
            0x0e => Some(Opcode::JmpIfFalse),
            0x0f => Some(Opcode::Call),
            0x10 => Some(Opcode::Ret),
            0x11 => Some(Opcode::LoadLocal),
            0x12 => Some(Opcode::StoreLocal),
            0x13 => Some(Opcode::Load),
            0x14 => Some(Opcode::Store),
            0x16 => Some(Opcode::Halt),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Value {
    Int(i64),
    Str(String)
}

pub struct VM {
    bytecode: Vec<u8>,
    ip: usize,
    stack: Vec<Value>,
    call_stack: Vec<usize>,
    global: HashMap<usize, Value>,
    local: HashMap<usize, Value>,
    debug_mode: bool
}

impl VM {
    pub fn new(bytecode: Vec<u8>, debug_mode: bool) -> VM {
        Self {
            bytecode,
            ip: 0,
            stack: Vec::new(),
            call_stack: Vec::new(),
            global: HashMap::new(),
            local: HashMap::new(),
            debug_mode
        }
    }

    fn validate_bytecode(&mut self) -> bool {
        if self.bytecode.len() < 4 {
            return false;
        }

        // Read the first 4 bytes as a u32 (little-endian)
        let mut buf = [0u8; 4];
        buf.copy_from_slice(&self.bytecode[0..4]);
        let magic = u32::from_le_bytes(buf);

        if magic == constants::MAGIC_NUMBER {
            self.ip += 4;  // Move the instruction pointer past the magic number
            true
        } else {
            false
        }
    }

    fn fetch_byte(&mut self) -> u8 {
        if self.ip >= self.bytecode.len() {
            eprintln!("VM Error: Out of bounds access attempted! The VM was looking for an opcode but found nothing.");
            process::exit(1);
        }
        let byte = self.bytecode[self.ip];
        self.ip += 1;
        byte
    }

    fn fetch_u64(&mut self) -> u64 {
        if self.ip + 8 >= self.bytecode.len() {
            eprintln!("VM Error: Out of bounds access attempted! The VM was looking for a value but found nothing (or not enough bytes).");
            process::exit(1);
        }
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&self.bytecode[self.ip..self.ip + 8]);
        self.ip += 8;
        u64::from_le_bytes(buf)
    }

    // pops two values from the stack and returns them as left and right, meant for binary operations
    fn pop_two_stack(&mut self) -> (Value, Value) {
        let right = self.stack.pop().unwrap_or_else(|| {
            eprintln!("VM Error: Stack underflow!");
            process::exit(1);
        });
        let left = self.stack.pop().unwrap_or_else(|| {
            eprintln!("VM Error: Stack underflow!");
            process::exit(1);
        });

        (left, right)
    }
    // once strings come this function will not apply for ADD opcodes
    fn binary_int_op<F>(&mut self, op: F, op_name: &str)
    where
        F: Fn(i64, i64) -> i64
    {
        let (left, right) = self.pop_two_stack();

        match (left, right) {
            (Value::Int(l), Value::Int(r)) => {
                if op_name == "division" && r == 0 {
                    eprintln!("VM Error: Cannot divide by zero.");
                    process::exit(1);
                } else if op_name == "modulus" && r == 0 {
                    eprintln!("VM Error: Cannot perform modulus by zero.");
                    process::exit(1);
                }

                self.stack.push(Value::Int(op(l, r)));
            }
            _ => {
                eprintln!("VM Error: Mismatched types on {} operation!", op_name);
                process::exit(1);
            }
        }
    }

    fn binary_cmp_op<F>(&mut self, op: F, op_name: &str)
    where
        F: Fn(i64, i64) -> bool,
    {
        let (left, right) = self.pop_two_stack();

        match (left, right) {
            (Value::Int(l), Value::Int(r)) => {
                self.stack.push(Value::Int(if op(l, r) { 1 } else { 0 }));
            }
            _ => {
                eprintln!("VM Error: Mismatched types for {} comparison!", op_name);
                process::exit(1);
            }
        }
    }

    pub fn interpret(&mut self) {
        if !self.validate_bytecode() {
            eprintln!("VM Error: Not a valid bytecode file!");
            process::exit(1);
        }

        loop {
            let opcode = self.fetch_byte();
            match Opcode::from_u8(opcode) {
                Some(Opcode::PushInt) => {
                    let value = self.fetch_u64();
                    self.stack.push(Value::Int(value.try_into().unwrap()));
                }
                Some(Opcode::Add) => {
                    let (left, right) = self.pop_two_stack();
                    
                    match (left, right) {
                        (Value::Int(left_val), Value::Int(right_val)) => {
                            self.stack.push(Value::Int(left_val + right_val));
                        }
                        (Value::Str(left_str), Value::Str(right_str)) => {
                            self.stack.push(Value::Str(format!("{}{}", left_str, right_str)));
                        }
                        _ => {
                            eprintln!("VM Error: Mismatched types on addition operation!");
                            process::exit(1);
                        }
                    }
                }
                Some(Opcode::Sub) => self.binary_int_op(|a , b| a - b, "subtraction"),
                Some(Opcode::Mul) => self.binary_int_op(|a, b| a * b, "multiplication"),
                Some(Opcode::Div) => self.binary_int_op(|a, b| a / b, "division"),
                Some(Opcode::Mod) => self.binary_int_op(|a, b| a % b, "modulus"),
                Some(Opcode::Neg) => {
                    let value = self.stack.pop().expect("VM Error: Stack underflow!");

                    match value {
                        Value::Int(val) => {
                            self.stack.push(Value::Int(0 - val));
                        }
                        _ => {
                            eprintln!("VM Error: Unsupported type for NEG operation, only numbers can be turned into negative values.");
                            process::exit(1);
                        }
                    }
                }
                Some(Opcode::Eq) => self.binary_cmp_op(|a, b| a == b, "EQ"),
                Some(Opcode::Neq) => self.binary_cmp_op(|a, b| a != b, "NEQ"),
                Some(Opcode::Lt) => self.binary_cmp_op(|a, b| a < b, "LT"),
                Some(Opcode::Gt) => self.binary_cmp_op(|a, b| a > b, "GT"),
                Some(Opcode::Jmp) => {
                    self.ip = self.fetch_u64().try_into().expect("VM Error: Attempted to do JMP operation, but converting the address into a usize failed!");
                }
                Some(Opcode::JmpIfTrue) => {
                    let address = self.fetch_u64();
                    let condition = self.stack.pop().expect("VM Error: Stack underflow!");
                    if condition != Value::Int(0) {
                        self.ip = address.try_into().expect("VM Error: Attempted to do JMP_IF_TRUE operation, but converting the address into a usize failed!");
                    }
                }
                Some(Opcode::JmpIfFalse) => {
                    let address = self.fetch_u64();
                    let condition = self.stack.pop().expect("VM Error: Stack underflow!");
                    if condition == Value::Int(0) {
                        self.ip = address.try_into().expect("VM Error: Attempted to do JMP_IF_FALSE operation, but converting the address to a usize failed!");
                    }
                }
                Some(Opcode::Call) => {
                    let address: usize = self.fetch_u64().try_into().expect("VM Error: Attempted to do CALL operation, but converting the address into a usize failed!");
                    self.call_stack.push(self.ip);
                    self.ip = address;
                }
                Some(Opcode::Ret) => {
                    let address = self.call_stack.pop().expect("VM Error: Call stack underflow! RET operation failed.");
                    self.ip = address;
                }
                Some(Opcode::LoadLocal) => {
                    let index: usize = self.fetch_u64().try_into().expect("VM Error: Attempted to do LOAD_LOCAL operation, but converting the variable name into a usize failed!");
                    let value = self.local.get(&index).expect("VM Error: Tried to load a local variable that does not exist!");
                    self.stack.push(value.clone());
                }
                Some(Opcode::StoreLocal) => {
                    let index: usize = self.fetch_u64().try_into().expect("VM Error: Attempted to do STORE operation, but converting the variable name into a usize failed!");
                    self.local.insert(index, self.stack.pop().expect("VM Error: Stack underflow!"));
                }
                Some(Opcode::Load) => {
                    let index: usize = self.fetch_u64().try_into().expect("VM Error: Attempted to do LOAD operation, but converting the variable name into a usize failed!");
                    let value = self.global.get(&index).expect("VM Error: Tried to load a variable that does not exist!");
                    self.stack.push(value.clone());
                }
                Some(Opcode::Store) => {
                    let index: usize = self.fetch_u64().try_into().expect("VM Error: Attempted to do STORE operation, but converting the variable name into a usize failed!");
                    self.global.insert(index, self.stack.pop().expect("VM Error: Stack underflow!"));
                }
                Some(Opcode::Halt) => {
                    if self.debug_mode {
                        println!("DEBUG: Process halted! Halt-time statistics printing:");
                        println!("DEBUG: Stack: {:#?}", self.stack);
                        println!("DEBUG: Global variable stack: {:#?}", self.global);
                    }

                    process::exit(0);
                }
                None => {
                    eprintln!("VM Error: Expected opcode, received: {:x}", opcode);
                    process::exit(1);
                }
                _ => {
                    todo!();
                }
            }
        }
    }
}