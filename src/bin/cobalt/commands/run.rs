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
    bytecode.push(constants::HALT);

    println!("{:#?}", bytecode);

    let mut vm = VM::new(bytecode, true);
    vm.interpret();
}