use crate::syntax::{
	SyntaxKind,
	Syntax,
	ExpressionKind,
	Expression
};

pub struct ParenthesisedExpression {
	content: Box<dyn Expression>
}

impl Syntax for ParenthesisedExpression {
	fn get_syntax_kind(&self) -> SyntaxKind { SyntaxKind::Expression }
}

impl Expression for ParenthesisedExpression {
	fn get_expression_kind(&self) -> ExpressionKind { ExpressionKind::Parenthesised }
}

impl ParenthesisedExpression {
	pub fn new(content: Box<dyn Expression>) -> Self {
		Self {
			content
		}
	}
}
