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

pub struct UnaryExpression {
	operand: Box<dyn Expression>,
	kind: UnaryExpressionKind
}

impl Syntax for UnaryExpression {
	fn get_syntax_kind(&self) -> SyntaxKind { SyntaxKind::Expression }

	fn print(&self, indentation: String) {
		match self.kind {
			UnaryExpressionKind::Identity => println!("+"),
			UnaryExpressionKind::Negation => println!("-")
		}
		self.operand.print(indentation + "  ");
	}
}

impl Expression for UnaryExpression {
	fn get_expression_kind(&self) -> ExpressionKind { ExpressionKind::Unary }
}

impl UnaryExpression {
	pub fn new(operand: Box<dyn Expression>, kind: UnaryExpressionKind) -> Self {
		Self {
			operand,
			kind
		}
	}

	pub fn get_binary_expression_kind(&self) -> UnaryExpressionKind { self.kind }
}
