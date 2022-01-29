use wasm_bindgen::prelude::*;
use roost::lexer::Lexer;
use roost::parser::Parser;
use roost::interpreter::Interpreter;

#[wasm_bindgen(raw_module = "../roost.worker")]
extern {
    pub fn print(message: String);
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
        }).collect();

        let line1 = if $error.location.line > 1 {
            format!("<br>&ensp;<span class=\"dark-gray\">{} | </span>{}", pad_three($error.location.line - 1), lines[$error.location.line - 2])
        } else { String::new() };
        let line2 = format!("&ensp;<span class=\"dark-gray\">{} | </span>{}", pad_three($error.location.line), lines[$error.location.line - 1]);
        let line3 = if $error.location.line < lines.len() {
            format!("<br>&ensp;<span class=\"dark-gray\">{} | </span>{}", pad_three($error.location.line + 1), lines[$error.location.line])
        } else { String::new() };

        let marker = format!("{}<span class=\"red bold\">^</span>", "&ensp;".repeat($error.location.column + 6));

        print(format!(
            "<span class=\"bold cyan\">{:?}</span><span class=\"bold\"> at {}:{}:{}</span><br>{}<br>{}<br>{}{}<br><br><span class=\"bold\">{}</span>",
            $error.kind,
            $error.location.filename,
            $error.location.line,
            $error.location.column,
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
            message.replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('\n', "<br>")
                .replace('\t', "&emsp;&emsp;&emsp;&emsp;")
                .replace(' ', "&ensp;")
        )
    });
    match interpreter.run() {
        Ok(_) => {},
        Err(e) => exit!(e, code),
    }
}
