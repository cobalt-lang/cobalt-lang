// BYTECODE START CODE

pub const MAGIC_NUMBER: u32 = 0xc0bac0de;
pub const MAGIC_NUMBER_U8: [u8; 4] = MAGIC_NUMBER.to_le_bytes();

// OPCODES - STACK MANIPULATION
pub const PUSH_INT: u8 = 0x01; // PUSH_INT 4
pub const PUSH_STR: u8 = 0x02; // PUSH_STR "hello world!"
pub const POP: u8 = 0x03; // POP

// ARITHMETIC
pub const ADD: u8 = 0x04; // PUSH_INT 5, PUSH_INT 3, ADD, this pops 5 and 3 and pushes 8 to the stack
pub const SUB: u8 = 0x05; // SUB
pub const MUL: u8 = 0x06; // MUL
pub const DIV: u8 = 0x07; // DIV
pub const MOD: u8 = 0x15; // MOD

// COMPARISONS
pub const EQ: u8 = 0x08; // PUSH INT 5, PUSH_INT 5, EQ
pub const NEQ: u8 = 0x09; // PUSH INT 5, PUSH_INT 3, NEQ
pub const LT: u8 = 0x0a; // PUSH_INT 3, PUSH_INT 5, LT
pub const GT: u8 = 0x0b; // PUSH_INT 5, PUSH_INT 3, GT

// CONTROL FLOW
pub const JMP: u8 = 0x0c; // JMP <address>, The program starts at address 0, each new line/newopcode is a new address.
pub const JMP_IF_TRUE: u8 = 0x0d; // PUSH_INT 1, JMP_IF_TRUE <address>, 1 is a truthy value
pub const JMP_IF_FALSE: u8 = 0x0e; // PUSH_INT 0, JMP_IF_FALSE <address>, 0 is a falsy value
pub const CALL: u8 = 0x0f; // CALL 0

// FUNCTION RELATED

pub const RET: u8 = 0x10; // PUSH_INT 5, RET
pub const LOAD_LOCAL: u8 = 0x11; // LOAD_LOCAL 0
pub const STORE_LOCAL: u8 = 0x12; // STORE_LOCAL <TYPE> 0

// MEMORY ACCESS (VARIABLE STORAGE)

pub const LOAD: u8 = 0x13; // LOAD 0
pub const STORE: u8 = 0x14; // STORE <TYPE> 0

// PROGRAM RELATED

pub const HALT: u8 = 0x16; // HALT