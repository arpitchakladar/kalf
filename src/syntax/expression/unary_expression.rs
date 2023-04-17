use crate::syntax::{
	SyntaxKind,
	Syntax,
	ExpressionKind,
	Expression
};

#[derive(Clone, Copy)]
pub enum UnaryExpressionKind {
	Identity,
	Negation
}

pub struct UnaryExpression<'a> {
	operand: Box<dyn Expression + 'a>,
	kind: UnaryExpressionKind
}

impl Syntax for UnaryExpression<'_> {
	fn get_syntax_kind(&self) -> SyntaxKind { SyntaxKind::Expression }

	fn print(&self, indentation: usize) {
		match self.kind {
			UnaryExpressionKind::Identity => println!("+"),
			UnaryExpressionKind::Negation => println!("-")
		}
		self.operand.print(indentation + 1);
	}
}

impl Expression for UnaryExpression<'_> {
	fn get_expression_kind(&self) -> ExpressionKind { ExpressionKind::Unary }
}

impl<'a> UnaryExpression<'a> {
	pub fn new(operand: Box<dyn Expression + 'a>, kind: UnaryExpressionKind) -> Self {
		Self {
			operand,
			kind
		}
	}

	pub fn get_binary_expression_kind(&self) -> UnaryExpressionKind { self.kind }
}
