use crate::syntax::{
	Expression
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UnaryExpressionKind {
	Identity,
	Negation
}

#[derive(Clone)]
pub struct UnaryExpression<'a> {
	operand: Box<Expression<'a>>,
	kind: UnaryExpressionKind
}

impl<'a> UnaryExpression<'a> {
	pub fn new(operand: Box<Expression<'a>>, kind: UnaryExpressionKind) -> Self {
		Self {
			operand,
			kind
		}
	}

	pub fn get_operand(&self) -> &Expression<'a> {
		&self.operand
	}

	pub fn get_kind(&self) -> UnaryExpressionKind {
		self.kind
	}
}
