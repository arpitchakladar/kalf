use std::cell::Cell;
use crate::syntax::{
	Syntax,
	Expression,
	BinaryExpressionContent,
	BinaryExpression,
	UnaryExpressionContent,
	UnaryExpression,
	LiteralExpressionContent,
	LiteralExpression,
	ParenthesisedExpression
};
use crate::lexing::Token;

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

	fn parse_expression(&'a self) -> Expression {
		if let Some(binary_expression) = self.parse_binary_expression() {
			return binary_expression;
		}

		panic!("Parsing failed")
	}

	fn parse_non_binary_expression(&'a self) -> Expression {
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

	fn parse_parenthesised_expression(&'a self) -> Option<Expression> {
		match self.get_current_token() {
			Token::OpenParenthesis(..) => {
				let content = self.parse_operand_expression();
				match self.get_current_token() {
					Token::CloseParenthesis(..) => {
						self.increment_index();
						Some(Expression::Parenthesised(ParenthesisedExpression::new(content)))
					},
					_ => None
				}
			},
			_ => None
		}
	}

	fn parse_literal_expression(&'a self) -> Option<Expression> {
		let current_token = self.get_current_token();

		Some(Expression::Literal(match current_token {
			Token::StringLiteral(..) => {
				self.increment_index();
				LiteralExpression::String(LiteralExpressionContent::new(current_token))
			},
			Token::CharacterLiteral(..) => {
				self.increment_index();
				LiteralExpression::Character(LiteralExpressionContent::new(current_token))
			},
			Token::IntegerLiteral(..) => {
				self.increment_index();
				LiteralExpression::Integer(LiteralExpressionContent::new(current_token))
			},
			Token::FloatingPointLiteral(..) => {
				self.increment_index();
				LiteralExpression::FloatingPoint(LiteralExpressionContent::new(current_token))
			},
			_ => return None
		}))
	}
	
	fn parse_operand_expression(&'a self) -> Box<Expression> {
		self.increment_index();
		Box::new(self.parse_expression())
	}

	fn parse_unary_expression(&'a self) -> Option<Expression> {
		Some(Expression::Unary(match self.get_current_token() {
			Token::PlusOperator(..) => UnaryExpression::Identity(UnaryExpressionContent::new(self.parse_operand_expression())),
			Token::MinusOperator(..) => UnaryExpression::Negation(UnaryExpressionContent::new(self.parse_operand_expression())),
			_ => return None
		}))
	}

	fn parse_binary_expression(&'a self) -> Option<Expression> {
		let left_operand = self.parse_non_binary_expression();

		Some(Expression::Binary(match self.get_current_token() {
			Token::PlusOperator(..) => BinaryExpression::Addition(BinaryExpressionContent::new(Box::new(left_operand), self.parse_operand_expression())),
			Token::MinusOperator(..) => BinaryExpression::Substraction(BinaryExpressionContent::new(Box::new(left_operand), self.parse_operand_expression())),
			Token::SlashOperator(..) => BinaryExpression::Division(BinaryExpressionContent::new(Box::new(left_operand), self.parse_operand_expression())),
			Token::StarOperator(..) => BinaryExpression::Multiplication(BinaryExpressionContent::new(Box::new(left_operand), self.parse_operand_expression())),
			Token::PercentageOperator(..) => BinaryExpression::Modulo(BinaryExpressionContent::new(Box::new(left_operand), self.parse_operand_expression())),
			_ => return Some(left_operand)
		}))
	}
}
