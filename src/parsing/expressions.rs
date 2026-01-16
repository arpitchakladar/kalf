use std::rc::Rc;
use crate::parsing::Parser;
use crate::syntax::{
	Expression,
	BinaryExpressionKind,
	BinaryExpression,
	UnaryExpressionKind,
	UnaryExpression,
	LiteralExpressionKind,
	LiteralExpression,
	ParenthesisedExpression
};
use crate::lexing::TokenKind;

impl<'a> Parser<'a> {
	pub(in crate::parsing) fn parse_expression(&self) -> Expression<'_> {
		if let Some(binary_expression) = self.parse_binary_expression() {
			return binary_expression;
		}

		panic!("Parsing failed")
	}

	fn parse_parenthesised_expression(&self) -> Option<Expression<'_>> {
		match self.current_token().kind() {
			TokenKind::OpenParenthesis => {
				self.increment_index();
				let content = self.parse_expression();

				match self.current_token().kind() {
					TokenKind::CloseParenthesis => {
						self.increment_index();
						Some(Expression::Parenthesised(ParenthesisedExpression::new(Rc::new(content))))
					},
					_ => panic!("Unclosed delimiter.")
				}
			},
			_ => None
		}
	}

	fn parse_literal_expression(&self) -> Option<Expression<'_>> {
		let current_token = self.current_token();

		let literal_expression_kind = match current_token.kind() {
			TokenKind::StringLiteral => LiteralExpressionKind::String,
			TokenKind::CharacterLiteral => LiteralExpressionKind::Character,
			TokenKind::IntegerLiteral => LiteralExpressionKind::Integer,
			TokenKind::FloatingPointLiteral => LiteralExpressionKind::FloatingPoint,
			_ => return None
		};

		self.increment_index();

		Some(Expression::Literal(LiteralExpression::new(current_token, literal_expression_kind)))
	}

	fn parse_unary_expression(&self) -> Option<Expression<'_>> {
		let unary_expression_kind = match self.current_token().kind() {
			TokenKind::PlusOperator => UnaryExpressionKind::Identity,
			TokenKind::MinusOperator => UnaryExpressionKind::Negation,
			_ => return None
		};

		self.increment_index();

		Some(
			Expression::Unary(
				UnaryExpression::new(
					Rc::new(self.parse_expression()),
					unary_expression_kind
				)
			)
		)
	}

	fn parse_non_binary_expression(&self) -> Expression<'_> {
		if let Some(parenthesised_expression) = self.parse_parenthesised_expression() {
			return parenthesised_expression;
		}

		if let Some(literal_expression) = self.parse_literal_expression() {
			return literal_expression;
		}

		if let Some(unary_expression) = self.parse_unary_expression() {
			return unary_expression;
		}

		panic!("Parsing failed");
	}

	fn parse_binary_expression(&self) -> Option<Expression<'_>> {
		let left_operand = self.parse_non_binary_expression();

		let binary_expression_kind = match self.current_token().kind() {
			TokenKind::PlusOperator => BinaryExpressionKind::Addition,
			TokenKind::MinusOperator => BinaryExpressionKind::Substraction,
			TokenKind::SlashOperator => BinaryExpressionKind::Division,
			TokenKind::StarOperator => BinaryExpressionKind::Multiplication,
			TokenKind::PercentageOperator => BinaryExpressionKind::Modulo,
			_ => return Some(left_operand)
		};

		self.increment_index();
		let right_operand = self.parse_expression();

		if let Expression::Binary(ref right_operand) = right_operand {
			if binary_expression_kind.precedence() > right_operand.kind().precedence() {
				let left_operand = Rc::new(
					Expression::Binary(
						BinaryExpression::new(
							Rc::new(left_operand),
							right_operand.left_operand_rc(),
							binary_expression_kind
						)
					)
				);

				return Some(
					Expression::Binary(
						BinaryExpression::new(
							left_operand,
							right_operand.right_operand_rc(),
							right_operand.kind()
						)
					)
				);
			}
		}

		Some(
			Expression::Binary(
				BinaryExpression::new(
					Rc::new(left_operand),
					Rc::new(right_operand),
					binary_expression_kind
				)
			)
		)
	}
}
