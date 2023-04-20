use crate::syntax::{
	BinaryExpression,
	UnaryExpression,
	LiteralExpression,
	ParenthesisedExpression
};

pub enum Expression<'a> {
	Binary(BinaryExpression<'a>),
	Unary(UnaryExpression<'a>),
	Literal(LiteralExpression<'a>),
	Parenthesised(ParenthesisedExpression<'a>)
}
