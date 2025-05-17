// OPCODES - STACK MANIPULATION
pub const PUSH: u8 = 0x0001;
pub const POP: u8 = 0x0002;

// ARITHMETIC
pub const ADD: u8 = 0x0003;
pub const SUB: u8 = 0x0004;
pub const MUL: u8 = 0x0005;
pub const DIV: u8 = 0x0006;
pub const MOD: u8 = 0x0014;

// COMPARISONS
pub const EQ: u8 = 0x0007;
pub const NEQ: u8 = 0x0008;
pub const LT: u8 = 0x0009;
pub const GT: u8 = 0x000a;

// CONTROL FLOW
pub const JMP: u8 = 0x000b;
pub const JMP_IF_TRUE: u8 = 0x000c;
pub const JMP_IF_FALSE: u8 = 0x000d;
pub const CALL: u8 = 0x000e;

// FUNCTION RELATED

pub const RET: u8 = 0x000f;
pub const LOAD_LOCAL: u8 = 0x0010;
pub const STORE_LOCAL: u8 = 0x0011;

// MEMORY ACCESS (VARIABLE STORAGE)

pub const LOAD: u8 = 0x0012;
pub const STORE: u8 = 0x0013;

// PROGRAM RELATED

pub const HALT: u8 = 0x0015;