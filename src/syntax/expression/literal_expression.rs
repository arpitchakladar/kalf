use crate::lexing::TokenContent;

pub enum LiteralExpression<'a> {
	Integer(LiteralExpressionContent<'a>),
	FloatingPoint(LiteralExpressionContent<'a>),
	Character(LiteralExpressionContent<'a>),
	String(LiteralExpressionContent<'a>)
}

pub struct LiteralExpressionContent<'a> {
	token: &'a TokenContent<'a>
}

impl<'a> LiteralExpressionContent<'a> {
	pub fn new(token: &'a TokenContent<'a>) -> Self {
		Self {
			token
		}
	}

	pub fn get_token(&self) -> &'a TokenContent<'a> {
		self.token
	}
}
