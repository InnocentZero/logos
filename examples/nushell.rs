//! An example of nushell syntax being lexed by logos
//!
//! Usage:
//!     cargo run --example nushell <path/to/file>
//!
//! Example:
//!     cargo run --example nushell examples/example.json

/* ANCHOR: all */
use logos::{Lexer, Logos, Span};

use std::env;
use std::fs;

type Error = (String, Span);

type Result<T> = std::result::Result<T, Error>;

/* ANCHOR: tokens */
/// All meaningful tokens.
///
/// > NOTE: regexes for [`Token::Number`] and [`Token::String`] may not
/// > catch all possible values, especially for strings. If you find
/// > errors, please report them so that we can improve the regex.
#[derive(Debug, Logos)]
#[logos(skip r"[ \t\r\f]+")]
enum Token<'source> {
    #[regex(r#""([^"\\]|\\["\\bnfrt])*""#, |lex| lex.slice(), priority = 20)]
    String(&'source str),

    #[regex(r#"'[^']*'"#, |lex| lex.slice(), priority = 20)]
    SingleQuoted(&'source str),

    #[regex(r#"`[^`]*`"#, |lex| lex.slice(), priority = 20)]
    BareWord(&'source str),

    #[regex(r"-?(?:0|[1-9]\d*)", |lex| lex.slice().parse::<i64>().unwrap(), priority = 3)]
    Int(i64),

    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Float(f64),

    #[regex(r"\n", |_| '\n')]
    Newline(char),
}
/* ANCHOR_END: tokens */

/* ANCHOR: values */
/// Represent any valid JSON value.
#[derive(Debug)]
enum Value<'source> {
    /// Any floating point number.
    Float(f64),
    // Any integer
    Int(i64),
    /// Any quoted string.
    String(&'source str),
    /// Any single quoted string.
    SingleQuoted(&'source str),
    /// Any single quoted string.
    BareWord(&'source str),
    /// Newline
    Newline(char),
}
/* ANCHOR_END: values */

/* ANCHOR: value */
/// Parse a token stream into a JSON value.
fn parse_value<'source>(
    lexer: &mut Lexer<'source, Token<'source>>,
) -> Result<Value<'source>> {
    if let Some(token) = lexer.next() {
        match token {
            Ok(Token::Float(n)) => Ok(Value::Float(n)),
            Ok(Token::String(s)) => Ok(Value::String(s)),
            Ok(Token::SingleQuoted(s)) => Ok(Value::SingleQuoted(s)),
            Ok(Token::BareWord(s)) => Ok(Value::BareWord(s)),
            Ok(Token::Newline(c)) => Ok(Value::Newline(c)),
            Ok(Token::Int(i)) => Ok(Value::Int(i)),
            _ => Err((
                "unexpected token here (context: value)".to_owned(),
                lexer.span(),
            )),
        }
    } else {
        Err(("EMPTY".to_owned(), lexer.span()))
    }
}
/* ANCHOR_END: value */

fn main() {
    let filename = env::args().nth(1).expect("Expected file argument");
    let src = fs::read_to_string(&filename).expect("Failed to read file");

    let mut lexer = Token::lexer(src.as_str());

    loop {
        match parse_value(&mut lexer) {
            Ok(value) => println!("{:#?}", value),
            Err((msg, span)) if msg == "EMPTY" => {
                break;
            }
            Err((msg, span)) => {
                use ariadne::{
                    ColorGenerator, Label, Report, ReportKind, Source,
                };

                let mut colors = ColorGenerator::new();

                let a = colors.next();

                Report::build(ReportKind::Error, &filename, 12)
                    .with_message("Invalid Lexeme".to_string())
                    .with_label(
                        Label::new((&filename, span))
                            .with_message(msg)
                            .with_color(a),
                    )
                    .finish()
                    .eprint((&filename, Source::from(&src)))
                    .unwrap();
                break;
            }
        }
    }
}
/* ANCHOR_END: all */
