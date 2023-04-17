use crate::syntax::{
	SyntaxKind,
	Syntax,
	ExpressionKind,
	Expression
};

pub struct ParenthesisedExpression<'a> {
	content: Box<dyn Expression + 'a>
}

impl Syntax for ParenthesisedExpression<'_> {
	fn get_syntax_kind(&self) -> SyntaxKind { SyntaxKind::Expression }

	fn print(&self, indentation: usize) {
		print!("(");
		self.content.print(indentation + 1);
		for _ in 0..indentation {
			print!("  ");
		}
		println!(")");
	}
}

impl Expression for ParenthesisedExpression<'_> {
	fn get_expression_kind(&self) -> ExpressionKind { ExpressionKind::Parenthesised }
}

impl<'a> ParenthesisedExpression<'a> {
	pub fn new(content: Box<dyn Expression + 'a>) -> Self {
		Self {
			content
		}
	}
}
