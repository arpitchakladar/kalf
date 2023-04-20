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
	pub fn get_precedence(&self) -> u8 {
		match self {
			BinaryExpressionKind::Addition |
			BinaryExpressionKind::Substraction => 1,

			BinaryExpressionKind::Multiplication |
			BinaryExpressionKind::Division |
			BinaryExpressionKind::Modulo => 2
		}
	}
}

#[derive(Clone)]
pub struct BinaryExpression<'a> {
	left_operand: Box<Expression<'a>>,
	right_operand: Box<Expression<'a>>,
	kind: BinaryExpressionKind
}

impl<'a> BinaryExpression<'a> {
	pub fn new(left_operand: Box<Expression<'a>>, right_operand: Box<Expression<'a>>, kind: BinaryExpressionKind) -> Self {
		Self {
			left_operand,
			right_operand,
			kind
		}
	}

	pub fn get_left_operand(&self) -> &Expression<'a> {
		&self.left_operand
	}

	pub fn get_right_operand(&self) -> &Expression<'a> {
		&self.right_operand
	}

	pub fn get_kind(&self) -> BinaryExpressionKind {
		self.kind
	}
}
