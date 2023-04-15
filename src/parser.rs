use crate::syntax::{
	SyntaxKind,
	Syntax,
	ExpressionKind,
	Expression,
	BinaryExpressionKind,
	BinaryExpression,
	UnaryExpressionKind,
	UnaryExpression,
	ParenthesisedExpression
};
use crate::lexing::{
	TokenKind,
	Token
};

pub struct Parser<'a> {
	tokens: &'a Vec<Token<'a>>,
	index: usize
}

impl<'a> Parser<'a> {
	pub fn new(tokens: &'a Vec<Token>, index: usize) -> Self {
		Self {
			tokens,
			index
		}
	}

	pub fn parse() -> Box<dyn Syntax> {
		todo!()
	}

	pub fn parseExpression() -> Box<dyn Expression> {
		todo!()
	}
}
