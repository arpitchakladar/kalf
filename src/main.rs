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

fn main() {
	let mut lexer = Lexer::new("42321312 23423 69.42 \"hello world\" 'a' (x + y * z) var1 = == + -");
	let mut tokens = Vec::new();

	loop {
		let token = lexer.lex().unwrap();
		if token.get_kind() == TokenKind::End {
			break;
		} else {
			tokens.push(token);
		}
	}

	let syntax_tree = SyntaxRoot::new(
		Box::new(BinaryExpression::new(
			Box::new(BinaryExpression::new(
				Box::new(LiteralExpression::new(tokens[0], LiteralExpressionKind::Integer)),
				Box::new(LiteralExpression::new(tokens[1], LiteralExpressionKind::Integer)),
				BinaryExpressionKind::Addition)
			),
			Box::new(BinaryExpression::new(
				Box::new(LiteralExpression::new(tokens[2], LiteralExpressionKind::Integer)),
				Box::new(BinaryExpression::new(
					Box::new(LiteralExpression::new(tokens[3], LiteralExpressionKind::Integer)),
					Box::new(LiteralExpression::new(tokens[4], LiteralExpressionKind::Integer)),
					BinaryExpressionKind::Addition)
				),
				BinaryExpressionKind::Substraction)
			),
			BinaryExpressionKind::Multiplication
		))
	);
	syntax_tree.print();
}
