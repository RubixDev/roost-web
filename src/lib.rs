use roost::interpreter::Interpreter;
use roost::io::Write;
use roost::lexer::Lexer;
use roost::parser::Parser;
use wasm_bindgen::prelude::*;

mod parse_ansi;

#[wasm_bindgen(raw_module = "../roost.worker")]
extern "C" {
    pub fn print(message: String);
    pub fn exit(code: i32);
}

macro_rules! print_error {
    ($error:expr, $code:expr) => {
        fn pad_three(num: usize) -> String {
            return format!("{: >3}", num).replace(' ', "&nbsp;");
        }

        let lines: Vec<String> = $code.split('\n').map(|line| {
            line.replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('\t', "&nbsp;&nbsp;&nbsp;&nbsp;")
                .replace(' ', "&nbsp;")
        }).collect();

        let line1 = if $error.span.start.line > 1 {
            format!("<br>&nbsp;<span class=\"dark-gray\">{} | </span>{}", pad_three($error.span.start.line - 1), lines[$error.span.start.line - 2])
        } else { String::new() };
        let line2 = format!("&nbsp;<span class=\"dark-gray\">{} | </span>{}", pad_three($error.span.start.line), lines[$error.span.start.line - 1]);
        let line3 = if $error.span.start.line < lines.len() {
            format!("<br>&nbsp;<span class=\"dark-gray\">{} | </span>{}", pad_three($error.span.start.line + 1), lines[$error.span.start.line])
        } else { String::new() };

        let marker = format!("{}<span class=\"red bold\">{}</span>", "&nbsp;".repeat($error.span.start.column + 6), "^".repeat($error.span.end.index - $error.span.start.index));

        print(format!(
            "<br><span class=\"bold cyan\">{:?}</span><span class=\"bold\"> at {}:{}:{}</span><br>{}<br>{}<br>{}{}<br><br><span class=\"red bold\">{}</span>",
            $error.kind,
            "playground.ro",
            $error.span.start.line,
            $error.span.start.column,
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
impl Write for Console {
    fn write(&mut self, buf: String) {
        print(parse_ansi::parse(
            buf.replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('\n', "<br>")
                .replace('\t', "&nbsp;&nbsp;&nbsp;&nbsp;")
                .replace(' ', "&nbsp;"),
        ));
    }
}

#[wasm_bindgen]
pub fn run(code: String) {
    let nodes = match Parser::new(Lexer::new(&code)).parse() {
        Ok(nodes) => nodes,
        Err(errors) => {
            for error in errors {
                print_error!(error, code);
            }
            return;
        }
    };

    if let Err(e) =
        Interpreter::new(&nodes, Console, Console, |code| exit(code)).run(true)
    {
        exit!(e, code);
    }
}
