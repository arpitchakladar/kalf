use std::rc::Rc;
use crate::syntax::{
	Expression
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UnaryExpressionKind {
	Identity,
	Negation
}

pub struct UnaryExpression<'a> {
	operand: Rc<Expression<'a>>,
	kind: UnaryExpressionKind
}

impl<'a> UnaryExpression<'a> {
	pub fn new(operand: Rc<Expression<'a>>, kind: UnaryExpressionKind) -> Self {
		Self {
			operand,
			kind
		}
	}

	pub fn operand(&self) -> &Expression<'a> {
		&self.operand
	}

	pub fn kind(&self) -> UnaryExpressionKind {
		self.kind
	}
}
