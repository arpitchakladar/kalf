use crate::syntax::{
	Expression,
	BinaryExpression,
	BinaryExpressionKind,
	UnaryExpression,
	UnaryExpressionKind,
	ParenthesisedExpression,
	LiteralExpression,
	LiteralExpressionKind
};

pub fn evaluate_expression(expression: &Expression) -> f64 {
	match expression {
		Expression::Binary(binary_expression) => evaluate_binary_expression(binary_expression),
		Expression::Unary(unary_expression) => evaluate_unary_expression(unary_expression),
		Expression::Parenthesised(parenthesised_expression) => evaluate_parenthesised_expression(parenthesised_expression),
		Expression::Literal(literal_expression) => evaluate_literal_expression(literal_expression)
	}
}

fn evaluate_binary_expression(binary_expression: &BinaryExpression) -> f64 {
	match binary_expression.kind() {
		BinaryExpressionKind::Addition => evaluate_expression(binary_expression.left_operand()) + evaluate_expression(binary_expression.right_operand()),
		BinaryExpressionKind::Substraction => evaluate_expression(binary_expression.left_operand()) - evaluate_expression(binary_expression.right_operand()),
		BinaryExpressionKind::Multiplication => evaluate_expression(binary_expression.left_operand()) * evaluate_expression(binary_expression.right_operand()),
		BinaryExpressionKind::Division => evaluate_expression(binary_expression.left_operand()) / evaluate_expression(binary_expression.right_operand()),
		BinaryExpressionKind::Modulo => evaluate_expression(binary_expression.left_operand()) % evaluate_expression(binary_expression.right_operand())
	}
}

fn evaluate_unary_expression(unary_expression: &UnaryExpression) -> f64 {
	match unary_expression.kind() {
		UnaryExpressionKind::Identity => evaluate_expression(unary_expression.operand()),
		UnaryExpressionKind::Negation => - evaluate_expression(unary_expression.operand())
	}
}

fn evaluate_parenthesised_expression(parenthesised_expression: &ParenthesisedExpression) -> f64 {
	evaluate_expression(parenthesised_expression.content())
}

fn evaluate_literal_expression(literal_expression: &LiteralExpression) -> f64 {
	match literal_expression.kind() {
		LiteralExpressionKind::Integer |
		LiteralExpressionKind::FloatingPoint => literal_expression.token().text().parse::<f64>().unwrap(),

		_ => panic!("Can only evaluate numbers")
	}
}
