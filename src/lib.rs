use wasm_bindgen::prelude::*;
use roost::lexer::Lexer;
use roost::parser::Parser;
use roost::interpreter::Interpreter;

mod parse_ansi;

#[wasm_bindgen(raw_module = "../roost.worker")]
extern {
    pub fn print(message: String);
    pub fn exit(code: i32);
}

macro_rules! exit {
    ($error:expr, $code:expr) => {{
        fn pad_three(num: usize) -> String {
            return format!("{: >3}", num).replace(' ', "&ensp;");
        }

        let lines: Vec<String> = $code.split('\n').map(|line| {
            line.replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('\t', "&emsp;&emsp;&emsp;&emsp;")
                .replace(' ', "&ensp;")
        }).collect();

        let line1 = if $error.start.line > 1 {
            format!("<br>&ensp;<span class=\"dark-gray\">{} | </span>{}", pad_three($error.start.line - 1), lines[$error.start.line - 2])
        } else { String::new() };
        let line2 = format!("&ensp;<span class=\"dark-gray\">{} | </span>{}", pad_three($error.start.line), lines[$error.start.line - 1]);
        let line3 = if $error.start.line < lines.len() {
            format!("<br>&ensp;<span class=\"dark-gray\">{} | </span>{}", pad_three($error.start.line + 1), lines[$error.start.line])
        } else { String::new() };

        let marker = format!("{}<span class=\"red bold\">{}</span>", "&ensp;".repeat($error.start.column + 6), "^".repeat($error.end.index - $error.start.index));

        print(format!(
            "<br><span class=\"bold cyan\">{:?}</span><span class=\"bold\"> at {}:{}:{}</span><br>{}<br>{}<br>{}{}<br><br><span class=\"red bold\">{}</span>",
            $error.kind,
            $error.start.filename,
            $error.start.line,
            $error.start.column,
            line1,
            line2,
            marker,
            line3,
            $error.message,
        ));
        return;
    }};
}

#[wasm_bindgen]
pub fn run(code: String) {
    let mut lexer = Lexer::new(&code, String::from("playground.ro"));
    let tokens = match lexer.scan() {
        Ok(tokens) => tokens,
        Err(e) => exit!(e, code),
    };

    let mut parser = Parser::new(&tokens);
    let nodes = match parser.parse() {
        Ok(nodes) => nodes,
        Err(e) => exit!(e, code),
    };

    let mut interpreter = Interpreter::new(nodes, |message| {
        print(
            parse_ansi::parse(message.replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('\n', "<br>")
                .replace('\t', "&emsp;&emsp;&emsp;&emsp;")
                .replace(' ', "&ensp;"))
        )
    }, exit);
    match interpreter.run() {
        Ok(_) => {},
        Err(e) => exit!(e, code),
    }
}
