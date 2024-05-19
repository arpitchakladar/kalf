mod lexing;
mod syntax;
mod parser;
mod diagnostic;
mod runtime;
mod types;

use std::fs;
use lexing::{
	TokenKind,
	Lexer
};
use parser::Parser;
use diagnostic::print_syntax;
use runtime::evaluate_syntax;

fn main() {
    let code = String::from_utf8(fs::read("tests/scripts/foo.kalf").unwrap()).unwrap();
	let lexer = Lexer::new(&code);
	let mut tokens = Vec::new();

	loop {
		let token = lexer.lex().unwrap();
		if token.kind() == TokenKind::End {
			break;
		} else {
			tokens.push(token);
		}
	}

	let parser = Parser::new(&tokens);
	let syntax = parser.parse();
	print_syntax(&syntax);
	println!("{}\n\nOUTPUT\n{}", code, evaluate_syntax(&syntax));
}
