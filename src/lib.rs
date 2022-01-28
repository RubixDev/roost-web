use wasm_bindgen::prelude::*;
use roost::lexer::Lexer;
use roost::parser::Parser;
use roost::interpreter::Interpreter;

#[wasm_bindgen(raw_module = "../roost.worker")]
extern {
    pub fn print(message: String);
}

#[wasm_bindgen]
pub fn run(code: String) {
    let mut lexer = Lexer::new(&code, String::from("playground.ro"));
    let tokens = match lexer.scan() {
        Ok(tokens) => tokens,
        Err(e) => { print(e.to_string()); return },
    };

    let mut parser = Parser::new(&tokens);
    let nodes = match parser.parse() {
        Ok(nodes) => nodes,
        Err(e) => { print(e.to_string()); return },
    };

    let mut interpreter = Interpreter::new(nodes, print);
    match interpreter.run() {
        Ok(_) => {},
        Err(e) => { print(e.to_string()); return },
    }
}
