use crate::syntax::{
	Expression
};

pub enum BinaryExpression<'a> {
	Addition(BinaryExpressionContent<'a>),
	Substraction(BinaryExpressionContent<'a>),
	Multiplication(BinaryExpressionContent<'a>),
	Division(BinaryExpressionContent<'a>),
	Modulo(BinaryExpressionContent<'a>)
}

pub struct BinaryExpressionContent<'a> {
	left_operand: Box<Expression<'a>>,
	right_operand: Box<Expression<'a>>
}

impl<'a> BinaryExpressionContent<'a> {
	pub fn new(left_operand: Box<Expression<'a>>, right_operand: Box<Expression<'a>>) -> Self {
		Self {
			left_operand,
			right_operand
		}
	}
}
