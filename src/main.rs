mod lexing;
mod syntax;
mod parser;

use lexing::{
	Token,
	Lexer
};
use parser::Parser;

fn main() {
	let lexer = Lexer::new("(10 * 32.2 - 12) / 12 - 12 + 13 % (9 - 6)");
	let mut tokens = Vec::new();

	loop {
		let token = lexer.lex().unwrap();
		if let Token::End = token {
			break;
		} else {
			tokens.push(token);
		}
	}

	let parser = Parser::new(&tokens);
	parser.parse();
}
