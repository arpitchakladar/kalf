use crate::lexing::Token;

#[derive(Clone, Copy, PartialEq, Eq)]
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

impl<'a> LiteralExpression<'a> {
	pub fn new(token: &'a Token<'a>, kind: LiteralExpressionKind) -> Self {
		Self {
			token,
			kind
		}
	}

	pub fn get_token(&self) -> &Token<'a> {
		&self.token
	}

	pub fn get_kind(&self) -> LiteralExpressionKind {
		self.kind
	}
}
