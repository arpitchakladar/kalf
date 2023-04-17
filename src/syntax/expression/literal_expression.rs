use crate::lexing::Token;
use crate::syntax::{
	SyntaxKind,
	Syntax,
	ExpressionKind,
	Expression
};

#[derive(Clone, Copy)]
pub enum LiteralExpressionKind {
	Integer,
	FloatingPoint,
	Character,
	String
}

pub struct LiteralExpression<'a> {
	token: &'a Token<'a>,
	kind: LiteralExpressionKind
}

impl Syntax for LiteralExpression<'_> {
	fn get_syntax_kind(&self) -> SyntaxKind { SyntaxKind::Expression }

	fn print(&self, _: usize) {
		println!("{}", self.token.get_text());
	}
}

impl Expression for LiteralExpression<'_> {
	fn get_expression_kind(&self) -> ExpressionKind { ExpressionKind::Literal }
}

impl<'a> LiteralExpression<'a> {
	pub fn new(token: &'a Token<'a>, kind: LiteralExpressionKind) -> Self {
		Self {
			token,
			kind
		}
	}

	pub fn get_literal_expression_kind(&self) -> LiteralExpressionKind { self.kind }
}
