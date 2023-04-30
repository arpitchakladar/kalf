mod lexing;
mod syntax;
mod parser;
mod diagnostic;
mod runtime;
mod types;

use lexing::{
	TokenKind,
	Lexer
};
use parser::Parser;
use diagnostic::print_syntax;
use runtime::evaluate_syntax;

fn main() {
	let code = "(10 * 32.2 - 12) / 12 - 4 + (44 - 23) + 5 - 12";
	let lexer = Lexer::new(code);
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
	println!("{} = {}", code, evaluate_syntax(&syntax));
}
