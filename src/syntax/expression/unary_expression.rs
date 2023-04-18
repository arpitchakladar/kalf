use crate::syntax::{
	Expression
};

pub enum UnaryExpression<'a> {
	Identity(UnaryExpressionContent<'a>),
	Negation(UnaryExpressionContent<'a>)
}

pub struct UnaryExpressionContent<'a> {
	operand: Box<Expression<'a>>
}

impl<'a> UnaryExpressionContent<'a> {
	pub fn new(operand: Box<Expression<'a>>) -> Self {
		Self {
			operand
		}
	}

	pub fn get_operand(&self) -> &Expression<'a> {
		&self.operand
	}
}
