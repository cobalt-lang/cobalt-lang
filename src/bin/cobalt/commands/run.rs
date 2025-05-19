use cobalt_lang::interpreter::vm::VM;
use cobalt_lang::interpreter::constants;
use cobalt_lang::utils::bytes;

// PLACEHOLDER FUNCTION FOR NOW, IT RUNS A PREMADE BYTECODE ARRAY

pub fn run() {
    let mut bytecode: Vec<u8> = vec![];
    bytecode.extend(constants::MAGIC_NUMBER_U8);
    bytecode.push(constants::PUSH_INT);
    bytecode.extend(bytes::to_little_endian(974));
    bytecode.push(constants::PUSH_INT);
    bytecode.extend(bytes::to_little_endian(26));
    bytecode.push(constants::ADD);
    bytecode.push(constants::STORE); // let x: int = 974 + 26 example
    bytecode.extend(bytes::to_little_endian(0));
    bytecode.push(constants::PUSH_INT);
    bytecode.extend(bytes::to_little_endian(174));
    bytecode.push(constants::LOAD);
    bytecode.extend(bytes::to_little_endian(0));
    bytecode.push(constants::ADD); // x + 174 example
    bytecode.push(constants::PUSH_INT);
    bytecode.extend(bytes::to_little_endian(1174));
    bytecode.push(constants::EQ); // 1174 == x example
    bytecode.push(constants::HALT);

    // THE RESULT OF THIS BYTECODE SHOULD BE 1

    let mut vm = VM::new(bytecode, true);
    vm.interpret();
}