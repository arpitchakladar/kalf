use crate::syntax::{
	BinaryExpression,
	UnaryExpression,
	LiteralExpression,
	ParenthesisedExpression
};

#[derive(Clone)]
pub enum Expression<'a> {
	Binary(BinaryExpression<'a>),
	Unary(UnaryExpression<'a>),
	Literal(LiteralExpression<'a>),
	Parenthesised(ParenthesisedExpression<'a>)
}
