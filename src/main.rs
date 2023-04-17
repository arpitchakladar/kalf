mod lexing;
mod syntax;
mod parser;

use lexing::{
	TokenKind,
	Token,
	Lexer
};
use syntax::{
	Syntax,
	SyntaxRoot,
	BinaryExpressionKind,
	BinaryExpression,
	LiteralExpressionKind,
	LiteralExpression
};
use parser::Parser;

fn main() {
	let mut lexer = Lexer::new("(10 * 32.2 - 12) / 12 - 12 + 13 % (9 - 6)");
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
	parser.parseExpression().print(0);
}
