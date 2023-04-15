mod lexing;

use lexing::{
	TokenKind,
	Token,
	Lexer
};

fn main() {
	let mut lexer = Lexer::new("42321312 23423 69.42 var1 = ==");
	let mut tokens = Vec::new();
	loop {
		let token = lexer.lex().unwrap();
		if token.get_kind() == TokenKind::End {
			break;
		} else {
			tokens.push(token);
		}
	}
	println!("{:?}", tokens);
}
