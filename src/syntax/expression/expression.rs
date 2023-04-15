use crate::syntax::{
	SyntaxKind,
	Syntax
};

pub enum ExpressionKind {
	Binary,
	Unary,
	Condition,
	Parenthesised
}

pub trait Expression: Syntax {
	fn get_expression_kind(&self) -> ExpressionKind;
}
