use std::{collections::HashMap, process};

use super::constants;
use crate::errors;

#[derive(Debug)]
pub enum Opcode {
    PushInt,
    PushStr,
    PushBool,
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,
    Not,
    Eq,
    Neq,
    Lt,
    Gt,
    Jmp,
    JmpIfTrue,
    JmpIfFalse,
    JmpIfTruePeek,
    JmpIfFalsePeek,
    Call,
    Ret,
    Load,
    Store,
    Halt,
}

impl Opcode {
    fn from_u8(byte: u8) -> Option<Self> {
        match byte {
            0x01 => Some(Opcode::PushInt),
            0x02 => Some(Opcode::PushStr),
            0x18 => Some(Opcode::PushBool),
            0x03 => Some(Opcode::Pop),
            0x04 => Some(Opcode::Add),
            0x05 => Some(Opcode::Sub),
            0x06 => Some(Opcode::Mul),
            0x07 => Some(Opcode::Div),
            0x15 => Some(Opcode::Mod),
            0x17 => Some(Opcode::Neg),
            0x19 => Some(Opcode::Not),
            0x08 => Some(Opcode::Eq),
            0x09 => Some(Opcode::Neq),
            0x0a => Some(Opcode::Lt),
            0x0b => Some(Opcode::Gt),
            0x0c => Some(Opcode::Jmp),
            0x0d => Some(Opcode::JmpIfTrue),
            0x0e => Some(Opcode::JmpIfFalse),
            0x1a => Some(Opcode::JmpIfTruePeek),
            0x1b => Some(Opcode::JmpIfFalsePeek),
            0x0f => Some(Opcode::Call),
            0x10 => Some(Opcode::Ret),
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
    Bool(bool),
    Str(String)
}

// TODO: add Ge and Le as options in the future
enum CmpOp {
    Eq,
    Neq,
    Lt,
    Gt
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
            errors::vm_err("Out of bounds access attempted! The VM was looking for an opcode but found nothing.", self.ip);
        }
        let byte = self.bytecode[self.ip];
        self.ip += 1;
        byte
    }

    fn fetch_u64(&mut self) -> u64 {
        if self.ip + 8 >= self.bytecode.len() {
            errors::vm_err("Out of bounds access attempted! The VM was looking for a value but found nothing (or not enough bytes).", self.ip);
        }
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&self.bytecode[self.ip..self.ip + 8]);
        self.ip += 8;
        u64::from_le_bytes(buf)
    }

    // pops two values from the stack and returns them as left and right, meant for binary operations
    fn pop_two_stack(&mut self) -> (Value, Value) {
        let right = self.stack.pop().unwrap_or_else(|| {
            errors::vm_err(errors::VMERR_STACK_UNDERFLOW, self.ip);
        });
        let left = self.stack.pop().unwrap_or_else(|| {
            errors::vm_err(errors::VMERR_STACK_UNDERFLOW, self.ip);
        });

        (left, right)
    }

    // gets type name from a Value
    fn get_type_name(&self, val: &Value) -> &str {
        match val {
            Value::Int(_) => "int",
            Value::Bool(_) => "bool",
            Value::Str(_) => "str",
        }
    }

    // once strings come, this function will not apply for ADD opcodes
    // for op_int, 1 = division, 2 = modulus, 0 = everything else, used because it's quicker than comparing op name for errors
    fn binary_int_op<F>(&mut self, op: F, op_name: &str, op_int: u8)
    where
        F: Fn(i64, i64) -> i64
    {
        let (left, right) = self.pop_two_stack();

        match (&left, &right) {
            (Value::Int(l), Value::Int(r)) => {
                if op_int == 1 && *r == 0 {
                    errors::vm_err("Cannot divide by zero.", self.ip);
                } else if op_int == 2 && *r == 0 {
                    errors::vm_err("Cannot perform modulus by zero.", self.ip);
                }

                self.stack.push(Value::Int(op(*l, *r)));
            }
            _ => {
                let err = format!("Mismatched or unsupported types on {} operation of type '{}' and '{}'.", op_name, self.get_type_name(&left), self.get_type_name(&right));
                errors::vm_err(&err, self.ip);
            }
        }
    }


    fn binary_cmp_op(&mut self, op: CmpOp, op_name: &str) {
        let (left, right) = self.pop_two_stack();
 
        let result = match (&left, &right, op) {
            (Value::Int(l), Value::Int(r), op) => match op {
                CmpOp::Eq => Value::Bool(l == r),
                CmpOp::Neq => Value::Bool(l != r),
                CmpOp::Lt => Value::Bool(l < r),
                CmpOp::Gt => Value::Bool(l > r),
            },
            (Value::Bool(l), Value::Bool(r), CmpOp::Eq) => Value::Bool(l == r),
            (Value::Bool(l), Value::Bool(r), CmpOp::Neq) => Value::Bool(l != r),
            _ => {
                // TODO: change this error in the future, it flows kind of weird
                // this error needs to be more specific about what kind of operation was done and what type that is not supported on
                // example: VM Error: ">" is not a supported operation for type bool.
                let err = format!("Mismatched or unsupported types on {} operation of type '{}' and '{}'.", op_name, self.get_type_name(&left), self.get_type_name(&right));
                errors::vm_err(&err, self.ip);
            }
        };

        self.stack.push(result);
    }

    pub fn interpret(&mut self) {
        if !self.validate_bytecode() {
            errors::vm_err("Not a valid bytecode file!", 4); // give a specific ip because this error just applies to the start of the file's first 4 bytes
        }

        loop {
            let opcode = self.fetch_byte();
            match Opcode::from_u8(opcode) {
                Some(Opcode::PushInt) => {
                    let value = self.fetch_u64();
                    self.stack.push(Value::Int(value.try_into().unwrap()));
                }
                Some(Opcode::PushBool) => {
                    let value = self.fetch_byte();

                    if value == 0 {
                        self.stack.push(Value::Bool(false))
                    } else {
                        self.stack.push(Value::Bool(true))
                    }
                }
                Some(Opcode::Pop) => {
                    self.stack.pop().unwrap_or_else(|| errors::vm_err(errors::VMERR_STACK_UNDERFLOW, self.ip));
                },
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
                            errors::vm_err("Mismatched types on an addition operation!", self.ip);
                        }
                    }
                }
                Some(Opcode::Sub) => self.binary_int_op(|a , b| a - b, "subtraction", 0),
                Some(Opcode::Mul) => self.binary_int_op(|a, b| a * b, "multiplication", 0),
                Some(Opcode::Div) => self.binary_int_op(|a, b| a / b, "division", 1),
                Some(Opcode::Mod) => self.binary_int_op(|a, b| a % b, "modulus", 2),
                Some(Opcode::Neg) => {
                    let value = self.stack.pop().unwrap_or_else(|| errors::vm_err(errors::VMERR_STACK_UNDERFLOW, self.ip));

                    match value {
                        Value::Int(val) => {
                            self.stack.push(Value::Int(0 - val));
                        }
                        _ => {
                            errors::vm_err("Unsupported type for NEG operation, only numbers can be turned into negative values.", self.ip);
                        }
                    }
                }
                Some(Opcode::Not) => {
                    let value = self.stack.pop().unwrap_or_else(|| errors::vm_err(errors::VMERR_STACK_UNDERFLOW, self.ip));

                    match value {
                        Value::Bool(val) => {
                            if val {
                                self.stack.push(Value::Bool(false));
                            } else {
                                self.stack.push(Value::Bool(true));
                            }
                        }
                        _ => {
                            errors::vm_err("Cannot apply NOT operation on a value that is not a boolean!", self.ip);
                        }
                    }
                }
                Some(Opcode::Eq) => self.binary_cmp_op(CmpOp::Eq, "=="),
                Some(Opcode::Neq) => self.binary_cmp_op(CmpOp::Neq, "!="),
                Some(Opcode::Lt) => self.binary_cmp_op(CmpOp::Lt, "<"),
                Some(Opcode::Gt) => self.binary_cmp_op(CmpOp::Gt, ">"),
                Some(Opcode::Jmp) => {
                    self.ip = self.fetch_u64().try_into().unwrap_or_else(|_| errors::vm_err("Attempted to do JMP operation, but converting the address into a usize failed!", self.ip));
                }
                Some(Opcode::JmpIfTrue) => {
                    let address = self.fetch_u64();
                    let condition = self.stack.pop().unwrap_or_else(|| errors::vm_err(errors::VMERR_STACK_UNDERFLOW, self.ip));
                    match condition {
                        Value::Bool(true) => {
                            self.ip = address.try_into().unwrap_or_else(|_| errors::vm_err("Converting address to usize failed!", self.ip));
                        }
                        Value::Bool(false) => { /* do nothing */ }
                        _ => {
                            let err = format!("JmpIfTrue expected a boolean condition, but got type '{}'.", self.get_type_name(&condition));
                            errors::vm_err(&err, self.ip);
                        }
                    }
                }
                Some(Opcode::JmpIfFalse) => {
                    let address = self.fetch_u64();
                    let condition = self.stack.pop().unwrap_or_else(|| errors::vm_err(errors::VMERR_STACK_UNDERFLOW, self.ip));
                    match condition {
                        Value::Bool(false) => {
                            self.ip = address.try_into().unwrap_or_else(|_| errors::vm_err("Converting address to usize failed!", self.ip));
                        }
                        Value::Bool(true) => { /* do nothing */ }
                        _ => {
                            let err = format!("JmpIfFalse expected a boolean condition, but got type '{}'.", self.get_type_name(&condition));
                            errors::vm_err(&err, self.ip);
                        }
                    }
                }
                Some(Opcode::JmpIfTruePeek) => {
                    let address = self.fetch_u64();
                    let condition = self.stack.last().unwrap_or_else(|| errors::vm_err(errors::VMERR_STACK_UNDERFLOW, self.ip));
                    match condition {
                        Value::Bool(true) => {
                            self.ip = address.try_into().unwrap_or_else(|_| errors::vm_err("Converting address to usize failed!", self.ip));
                        }
                        Value::Bool(false) => { /* do nothing */ }
                        _ => {
                            let err = format!("JmpIfTruePeek expected a boolean condition, but got type '{}'.", self.get_type_name(condition));
                            errors::vm_err(&err, self.ip);
                        }
                    }
                }
                Some(Opcode::JmpIfFalsePeek) => {
                    let address = self.fetch_u64();
                    let condition = self.stack.last().unwrap_or_else(|| errors::vm_err(errors::VMERR_STACK_UNDERFLOW, self.ip));
                    match condition {
                        Value::Bool(false) => {
                            self.ip = address.try_into().unwrap_or_else(|_| errors::vm_err("Converting address to usize failed!", self.ip));
                        }
                        Value::Bool(true) => { /* do nothing */ }
                        _ => {
                            let err = format!("JmpIfFalsePeek expected a boolean condition, but got type '{}'.", self.get_type_name(condition));
                            errors::vm_err(&err, self.ip);
                        }
                    }
                }
                Some(Opcode::Call) => {
                    let address: usize = self.fetch_u64().try_into().unwrap_or_else(|_| errors::vm_err("Attempted to do CALL operation, but converting the address into a usize failed!", self.ip));
                    self.call_stack.push(self.ip);
                    self.ip = address;
                }
                Some(Opcode::Ret) => {
                    let address = self.call_stack.pop().unwrap_or_else(|| errors::vm_err("Call stack underflow! RET operation failed.", self.ip));
                    self.ip = address;
                }
                Some(Opcode::Load) => {
                    let index: usize = self.fetch_u64().try_into().unwrap_or_else(|_| errors::vm_err("Attempted to do LOAD operation, but converting the variable name into a usize failed!", self.ip));
                    let value = self.global.get(&index).unwrap_or_else(|| errors::vm_err(format!("Tried to load variable at index '{}' that does not exist!", &index).as_str(), self.ip));
                    self.stack.push(value.clone());
                }
                Some(Opcode::Store) => {
                    let index: usize = self.fetch_u64().try_into().unwrap_or_else(|_| errors::vm_err("Attempted to do STORE operation, but converting the variable name into a usize failed!", self.ip));
                    self.global.insert(index, self.stack.pop().unwrap_or_else(|| errors::vm_err(errors::VMERR_STACK_UNDERFLOW, self.ip)));
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
                    let err = format!("Expected opcode, received: {:x}, at IP: {}", opcode, self.ip);
                    errors::vm_err(&err, self.ip);
                }
                _ => {
                    todo!("Unimplemented opcode: {:x}", opcode);
                }
            }
        }
    }
}