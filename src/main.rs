mod lexing;
mod syntax;
mod parser;
mod diagnostic;
mod runtime;

use lexing::{
	TokenKind,
	Lexer
};
use parser::Parser;
use diagnostic::print_syntax;
use runtime::evaluate_syntax;

fn main() {
	let lexer = Lexer::new("(10 * 32.2 - 12) / 12 - 12");
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
	println!("Value = {}", evaluate_syntax(&syntax));
}
