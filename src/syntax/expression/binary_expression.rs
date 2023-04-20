use std::rc::Rc;
use crate::syntax::{
	Expression
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BinaryExpressionKind {
	Addition,
	Substraction,
	Multiplication,
	Division,
	Modulo
}

impl BinaryExpressionKind {
	pub fn precedence(&self) -> u8 {
		match self {
			BinaryExpressionKind::Addition |
			BinaryExpressionKind::Substraction => 1,

			BinaryExpressionKind::Multiplication |
			BinaryExpressionKind::Division |
			BinaryExpressionKind::Modulo => 2
		}
	}
}

pub struct BinaryExpression<'a> {
	left_operand: Rc<Expression<'a>>,
	right_operand: Rc<Expression<'a>>,
	kind: BinaryExpressionKind
}

impl<'a> BinaryExpression<'a> {
	pub fn new(left_operand: Rc<Expression<'a>>, right_operand: Rc<Expression<'a>>, kind: BinaryExpressionKind) -> Self {
		Self {
			left_operand,
			right_operand,
			kind
		}
	}

	pub fn left_operand(&self) -> &Expression<'a> {
		&self.left_operand
	}

	pub fn left_operand_rc(&self) -> Rc<Expression<'a>> {
		self.left_operand.clone()
	}

	pub fn right_operand(&self) -> &Expression<'a> {
		&self.right_operand
	}

	pub fn right_operand_rc(&self) -> Rc<Expression<'a>> {
		self.right_operand.clone()
	}

	pub fn kind(&self) -> BinaryExpressionKind {
		self.kind
	}
}
