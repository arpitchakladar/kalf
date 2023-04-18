use crate::syntax::{
	Syntax,
	Expression,
	BinaryExpressionContent,
	BinaryExpression,
	UnaryExpressionContent,
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
	print_expression(parenthesised_expression.get_content(), indentation + 1);
	print_indentation(indentation);
	println!(")");
}

fn print_literal_expression(literal_expression: &LiteralExpression, _: usize) {
	match literal_expression {
		LiteralExpression::Integer(literal_expression_content) |
		LiteralExpression::FloatingPoint(literal_expression_content) |
		LiteralExpression::Character(literal_expression_content) |
		LiteralExpression::String(literal_expression_content) => println!("{}", literal_expression_content.get_token().get_text())
	}
}

fn print_unary_expression_content(operator: &str, unary_expression_content: &UnaryExpressionContent, indentation: usize) {
	print!("{} ", operator);
	print_expression(unary_expression_content.get_operand(), indentation + 1);
}

fn print_unary_expression(unary_expression: &UnaryExpression, indentation: usize) {
	match unary_expression {
		UnaryExpression::Identity(unary_expression_content) => print_unary_expression_content("+", unary_expression_content, indentation),
		UnaryExpression::Negation(unary_expression_content) => print_unary_expression_content("-", unary_expression_content, indentation)
	}
}

fn print_binary_expression_content(operator: &str, binary_expression_content: &BinaryExpressionContent, indentation: usize) {
	println!("{}", operator);
	print_indentation(indentation);
	print!("└ ");
	print_expression(binary_expression_content.get_left_operand(), indentation + 1);
	print_indentation(indentation);
	print!("└ ");
	print_expression(binary_expression_content.get_right_operand(), indentation + 1);
}

fn print_binary_expression(binary_expression: &BinaryExpression, indentation: usize) {
	match binary_expression {
		BinaryExpression::Addition(binary_expression_content) => print_binary_expression_content("+", binary_expression_content, indentation),
		BinaryExpression::Substraction(binary_expression_content) => print_binary_expression_content("-", binary_expression_content, indentation),
		BinaryExpression::Multiplication(binary_expression_content) => print_binary_expression_content("*", binary_expression_content, indentation),
		BinaryExpression::Division(binary_expression_content) => print_binary_expression_content("/", binary_expression_content, indentation),
		BinaryExpression::Modulo(binary_expression_content) => print_binary_expression_content("%", binary_expression_content, indentation)
	}
}

fn print_indentation(indentation: usize) {
	for _ in 0..indentation {
		print!("  ");
	}
}
