mod lexing;
mod syntax;
mod parser;
mod diagnostic;

use lexing::{
	TokenKind,
	Lexer
};
use parser::Parser;
use diagnostic::print_syntax;

fn main() {
	let lexer = Lexer::new("(10 * 32.2 - 12) / 12 - 12 + 13 % (-9 - -(6 * 2 + 2))");
	let mut tokens = Vec::new();

	loop {
		let token = lexer.lex().unwrap();
		if token.get_kind() == TokenKind::End {
			break;
		} else {
			tokens.push(token);
		}
	}

	let parser = Parser::new(&tokens);
	print_syntax(&parser.parse());
}
