use std::cell::Cell;
use crate::syntax::{
	Syntax,
	Expression,
	BinaryExpressionKind,
	BinaryExpression,
	UnaryExpressionKind,
	UnaryExpression,
	LiteralExpressionKind,
	LiteralExpression,
	ParenthesisedExpression
};
use crate::lexing::{
	Token,
	TokenKind
};

pub struct Parser<'a> {
	tokens: &'a Vec<Token<'a>>,
	index: Cell<usize>
}

impl<'a> Parser<'a> {
	pub fn new(tokens: &'a Vec<Token>) -> Self {
		Self {
			tokens,
			index: Cell::new(0)
		}
	}

	fn increment_index(&self) {
		self.index.set(self.index.get() + 1);
	}

	fn get_current_token(&self) -> &Token {
		if self.index.get() < self.tokens.len() {
			&self.tokens[self.index.get()]
		} else {
			&self.tokens[self.tokens.len() - 1]
		}
	}

	pub fn parse(&self) -> Box<Syntax> {
		Box::new(Syntax::Expression(self.parse_expression()))
	}

	fn parse_expression(&self) -> Expression {
		if let Some(binary_expression) = self.parse_binary_expression() {
			return binary_expression;
		}

		panic!("Parsing failed")
	}

	fn parse_parenthesised_expression(&self) -> Option<Expression> {
		match self.get_current_token().get_kind() {
			TokenKind::OpenParenthesis => {
				let content = self.parse_operand_expression();
				match self.get_current_token().get_kind() {
					TokenKind::CloseParenthesis => {
						self.increment_index();
						Some(Expression::Parenthesised(ParenthesisedExpression::new(Box::new(content))))
					},
					_ => panic!("Unclosed delimiter.")
				}
			},
			_ => None
		}
	}

	fn parse_literal_expression(&self) -> Option<Expression> {
		let current_token = self.get_current_token();

		let literal_expression_kind = match current_token.get_kind() {
			TokenKind::StringLiteral => LiteralExpressionKind::String,
			TokenKind::CharacterLiteral => LiteralExpressionKind::Character,
			TokenKind::IntegerLiteral => LiteralExpressionKind::Integer,
			TokenKind::FloatingPointLiteral => LiteralExpressionKind::FloatingPoint,
			_ => return None
		};

		self.increment_index();

		Some(Expression::Literal(LiteralExpression::new(current_token, literal_expression_kind)))
	}

	fn parse_operand_expression(&self) -> Expression {
		self.increment_index();
		self.parse_expression()
	}

	fn parse_unary_expression(&self) -> Option<Expression> {
		let unary_expression_kind = match self.get_current_token().get_kind() {
			TokenKind::PlusOperator => UnaryExpressionKind::Identity,
			TokenKind::MinusOperator => UnaryExpressionKind::Negation,
			_ => return None
		};

		Some(Expression::Unary(UnaryExpression::new(Box::new(self.parse_operand_expression()), unary_expression_kind)))
	}

	fn parse_non_binary_expression(&self) -> Expression {
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

	fn parse_binary_expression(&self) -> Option<Expression> {
		let left_operand = self.parse_non_binary_expression();

		let precedence;
		let binary_expression_kind = match self.get_current_token().get_kind() {
			TokenKind::PlusOperator => {
				precedence = 1;
				BinaryExpressionKind::Addition
			},
			TokenKind::MinusOperator => {
				precedence = 1;
				BinaryExpressionKind::Substraction
			},
			TokenKind::SlashOperator => {
				precedence = 2;
				BinaryExpressionKind::Division
			},
			TokenKind::StarOperator => {
				precedence = 2;
				BinaryExpressionKind::Multiplication
			},
			TokenKind::PercentageOperator => {
				precedence = 2;
				BinaryExpressionKind::Modulo
			},
			_ => return Some(left_operand)
		};

		let right_operand = self.parse_operand_expression();

		Some(Expression::Binary(BinaryExpression::new(Box::new(left_operand), Box::new(right_operand), binary_expression_kind)))
	}
}
