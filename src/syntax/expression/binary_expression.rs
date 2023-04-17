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

pub struct BinaryExpression<'a> {
	left_operand: Box<dyn Expression + 'a>,
	right_operand: Box<dyn Expression + 'a>,
	kind: BinaryExpressionKind
}

impl Syntax for BinaryExpression<'_> {
	fn get_syntax_kind(&self) -> SyntaxKind { SyntaxKind::Expression }

	fn print(&self, indentation: usize) {
		match self.kind {
			BinaryExpressionKind::Addition => println!("+"),
			BinaryExpressionKind::Substraction => println!("-"),
			BinaryExpressionKind::Multiplication => println!("*"),
			BinaryExpressionKind::Division => println!("/"),
			BinaryExpressionKind::Modulo => println!("%"),
		}
		for _ in 0..indentation {
			print!("  ");
		}
		print!("└ ");
		self.left_operand.print(indentation + 1);
		for _ in 0..indentation {
			print!("  ");
		}
		print!("└ ");
		self.right_operand.print(indentation + 1);
	}
}

impl Expression for BinaryExpression<'_> {
	fn get_expression_kind(&self) -> ExpressionKind { ExpressionKind::Binary }
}

impl<'a> BinaryExpression<'a> {
	pub fn new(left_operand: Box<dyn Expression + 'a>, right_operand: Box<dyn Expression + 'a>, kind: BinaryExpressionKind) -> Self {
		Self {
			left_operand,
			right_operand,
			kind
		}
	}

	pub fn get_binary_expression_kind(&self) -> BinaryExpressionKind { self.kind }
}
