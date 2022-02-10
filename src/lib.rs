use wasm_bindgen::prelude::*;
use roost::lexer::Lexer;
use roost::parser::Parser;
use roost::interpreter::{Interpreter, Exit};
use roost::io::Write;

mod parse_ansi;

#[wasm_bindgen(raw_module = "../roost.worker")]
extern {
    pub fn print(message: String);
    pub fn exit(code: i32);
}

macro_rules! print_error {
    ($error:expr, $code:expr) => {
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
        ))
    };
}

macro_rules! exit {
    ($error:expr, $code:expr) => {{
        print_error!($error, $code);
        return;
    }};
}

struct Console;
impl Console { pub fn new() -> Self { Console {} } }
impl Write for Console {
    fn write(&mut self, buf: String) {
        print(parse_ansi::parse(buf
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('\n', "<br>")
            .replace('\t', "&emsp;&emsp;&emsp;&emsp;")
            .replace(' ', "&ensp;")));
    }
}

struct Quit;
impl Exit for Quit {
    fn exit(&mut self, code: i32) { exit(code); }
}

#[wasm_bindgen]
pub fn run(code: String) {
    let nodes = match Parser::new_parse(Lexer::new(&code, String::from("playground.ro"))) {
        Ok(nodes) => nodes,
        Err(errors) => {
            for error in errors {
                print_error!(error, code);
            }
            return;
        },
    };

    match Interpreter::new_run(nodes, Console::new(), Quit {}) {
        Ok(_) => {},
        Err(e) => exit!(e, code),
    }
}
