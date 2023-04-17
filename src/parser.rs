use std::cell::Cell;
use crate::syntax::{
	SyntaxKind,
	Syntax,
	SyntaxRoot,
	ExpressionKind,
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
	TokenKind,
	Token
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

	pub fn parse(&self) -> SyntaxRoot {
		todo!()
	}

	pub fn parseExpression(&'a self) -> Box<dyn Expression + 'a> {
		if let Some(binary_expression) = self.parseBinaryExpression() {
			return binary_expression;
		}

		panic!("Parsing failed")
	}

	fn parseNonBinaryExpression(&'a self) -> Box<dyn Expression + 'a> {
		if let Some(parenthesised_expression) = self.parseParenthesisedExpression() {
			return parenthesised_expression;
		}

		if let Some(literal_expression) = self.parseLiteralExpression() {
			return literal_expression;
		}

		if let Some(unary_expression) = self.parseUnaryExpression() {
			return unary_expression;
		}

		panic!("Parsing failed");
	}

	fn parseParenthesisedExpression(&'a self) -> Option<Box<dyn Expression + 'a>> {
		if self.get_current_token().get_kind() == TokenKind::OpenParenthesis {
			self.increment_index();
			let content_expression = self.parseExpression();
			if self.get_current_token().get_kind() == TokenKind::CloseParenthesis {
				self.increment_index();
				return Some(Box::new(ParenthesisedExpression::new(content_expression)));
			}
		}

		None
	}

	fn parseLiteralExpression(&'a self) -> Option<Box<dyn Expression + 'a>> {
		let current_token = self.get_current_token();
		let literal_expression_kind = match current_token.get_kind() {
			TokenKind::StringLiteral => LiteralExpressionKind::String,
			TokenKind::CharacterLiteral => LiteralExpressionKind::Character,
			TokenKind::IntegerLiteral => LiteralExpressionKind::Integer,
			TokenKind::FloatingPointLiteral => LiteralExpressionKind::FloatingPoint,
			_ => return None
		};

		self.increment_index();

		Some(Box::new(LiteralExpression::new(current_token, literal_expression_kind)))
	}

	fn parseUnaryExpression(&'a self) -> Option<Box<dyn Expression + 'a>> {
		let unary_expression_kind = match self.get_current_token().get_kind() {
			TokenKind::PlusOperator => UnaryExpressionKind::Identity,
			TokenKind::MinusOperator => UnaryExpressionKind::Negation,
			_ => return None
		};

		self.increment_index();
		let operand = self.parseExpression();

		Some(Box::new(UnaryExpression::new(operand, unary_expression_kind)))
	}

	fn parseBinaryExpression(&'a self) -> Option<Box<dyn Expression + 'a>> {
		let left_operand = self.parseNonBinaryExpression();

		let binary_operation_kind = match self.get_current_token().get_kind() {
			TokenKind::PlusOperator => BinaryExpressionKind::Addition,
			TokenKind::MinusOperator => BinaryExpressionKind::Substraction,
			TokenKind::SlashOperator => BinaryExpressionKind::Division,
			TokenKind::StarOperator => BinaryExpressionKind::Multiplication,
			TokenKind::PercentageOperator => BinaryExpressionKind::Modulo,
			_ => return Some(left_operand)
		};

		self.increment_index();

		Some(Box::new(BinaryExpression::new(left_operand, self.parseExpression(), binary_operation_kind)))
	}
}
