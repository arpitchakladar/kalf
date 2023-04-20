use crate::syntax::{
	Syntax,
	Expression,
	BinaryExpressionKind,
	BinaryExpression,
	UnaryExpressionKind,
	UnaryExpression,
	LiteralExpression,
	ParenthesisedExpression
};

pub fn print_syntax(syntax: &Syntax) {
	match syntax {
		Syntax::Expression(expression) => print_expression(expression, 0)
	}
}

fn print_expression(expression: &Expression, indentation: usize) {
	match expression {
		Expression::Binary(binary_expression) => print_binary_expression(binary_expression, indentation),
		Expression::Unary(unary_expression) => print_unary_expression(unary_expression, indentation),
		Expression::Literal(literal_expression) => print_literal_expression(literal_expression, indentation),
		Expression::Parenthesised(parenthesised_expression) => print_parenthesised_expression(parenthesised_expression, indentation)
	}
}

fn print_parenthesised_expression(parenthesised_expression: &ParenthesisedExpression, indentation: usize) {
	print!("( ");
	print_expression(parenthesised_expression.content(), indentation + 1);
	print_indentation(indentation);
	println!(")");
}

fn print_literal_expression(literal_expression: &LiteralExpression, _: usize) {
	println!("{}", literal_expression.token().text());
}

fn print_unary_expression(unary_expression: &UnaryExpression, indentation: usize) {
	match unary_expression.kind() {
		UnaryExpressionKind::Identity => print!("+ "),
		UnaryExpressionKind::Negation => print!("- ")
	}

	print_expression(unary_expression.operand(), indentation + 1);
}

fn print_binary_expression(binary_expression: &BinaryExpression, indentation: usize) {
	match binary_expression.kind() {
		BinaryExpressionKind::Addition => println!("+"),
		BinaryExpressionKind::Substraction => println!("-"),
		BinaryExpressionKind::Multiplication => println!("*"),
		BinaryExpressionKind::Division => println!("/"),
		BinaryExpressionKind::Modulo => println!("%")
	}

	print_indentation(indentation);
	print!("└ ");
	print_expression(binary_expression.left_operand(), indentation + 1);
	print_indentation(indentation);
	print!("└ ");
	print_expression(binary_expression.right_operand(), indentation + 1);
}

fn print_indentation(indentation: usize) {
	for _ in 0..indentation {
		print!("  ");
	}
}
