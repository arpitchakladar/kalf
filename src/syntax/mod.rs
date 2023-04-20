mod expression;

pub use expression::{
	Expression,
	BinaryExpressionKind,
	BinaryExpression,
	UnaryExpressionKind,
	UnaryExpression,
	LiteralExpressionKind,
	LiteralExpression,
	ParenthesisedExpression
};

pub enum Syntax<'a> {
	Expression(Expression<'a>)
}
