use crate::syntax::{
	SyntaxKind,
	Syntax,
	ExpressionKind,
	Expression
};

#[derive(Clone, Copy)]
pub enum BinaryExpressionKind {
	Addition,
	Substraction,
	Multiplication,
	Division,
	Modulo
}

pub struct BinaryExpression {
	left_operand: Box<dyn Expression>,
	right_operand: Box<dyn Expression>,
	kind: BinaryExpressionKind
}

impl Syntax for BinaryExpression {
	fn get_syntax_kind(&self) -> SyntaxKind { SyntaxKind::Expression }

	fn print(&self, indentation: String) {
		match self.kind {
			BinaryExpressionKind::Addition => println!("+"),
			BinaryExpressionKind::Substraction => println!("-"),
			BinaryExpressionKind::Multiplication => println!("*"),
			BinaryExpressionKind::Division => println!("/"),
			BinaryExpressionKind::Modulo => println!("%"),
		}
		print!("{}└ ", &indentation);
		self.left_operand.print(indentation.clone() + "| ");
		print!("{}└ ", &indentation);
		self.right_operand.print(indentation + "  ");
	}
}

impl Expression for BinaryExpression {
	fn get_expression_kind(&self) -> ExpressionKind { ExpressionKind::Binary }
}

impl BinaryExpression {
	pub fn new(left_operand: Box<dyn Expression>, right_operand: Box<dyn Expression>, kind: BinaryExpressionKind) -> Self {
		Self {
			left_operand,
			right_operand,
			kind
		}
	}

	pub fn get_binary_expression_kind(&self) -> BinaryExpressionKind { self.kind }
}
