use crate::lexing::Token;

pub enum LiteralExpression<'a> {
	Integer(LiteralExpressionContent<'a>),
	FloatingPoint(LiteralExpressionContent<'a>),
	Character(LiteralExpressionContent<'a>),
	String(LiteralExpressionContent<'a>)
}

pub struct LiteralExpressionContent<'a> {
	token: &'a Token<'a>
}

impl<'a> LiteralExpressionContent<'a> {
	pub fn new(token: &'a Token<'a>) -> Self {
		Self {
			token
		}
	}
}
