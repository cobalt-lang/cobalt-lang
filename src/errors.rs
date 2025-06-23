// This file defines default formatting for error messages in the lexer, parser, generator, and VM.

use colored::*;
use std::process;

// COMMON VM ERRORS AS CONSTANTS

pub const VMERR_STACK_UNDERFLOW: &str = "Stack underflow!";
pub const VMERR_STACK_OVERFLOW: &str = "Stack overflow!";

pub fn vm_err(msg: &str, ip: usize) -> ! {
    eprintln!("{} {}\nIP: {}", "VM Error:".bold().red(), msg, ip);
    process::exit(1)
}

