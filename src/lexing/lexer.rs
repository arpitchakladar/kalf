use std::cell::Cell;
use crate::lexing::{
	Token,
	TokenKind
};

pub struct Lexer<'a> {
	index: Cell<usize>,
	text: &'a str
}

impl<'a> Lexer<'a> {
	pub fn new(text: &'a str) -> Self {
		Self {
			index: Cell::new(0),
			text
		}
	}

	pub fn lex(&self) -> Result<Token<'a>, &'static str> {
		if let Some(end_token) = self.lex_white_space() {
			return Ok(end_token);
		}

		if let Some(number_literal_token) = self.lex_number_literal() {
			return Ok(number_literal_token);
		}

		if let Some(string_literal_token) = self.lex_string_literal() {
			return Ok(string_literal_token);
		}

		if let Some(character_literal_token) = self.lex_character_literal() {
			return Ok(character_literal_token);
		}

		if let Some(keyword_or_identifier_token) = self.lex_keyword_and_identifier() {
			return Ok(keyword_or_identifier_token);
		}

		if let Some(operator_token) = self.lex_operator() {
			return Ok(operator_token);
		}

		Err("Unknown token.")
	}

	fn increment_index(&self) {
		self.index.set(self.index.get() + 1);
	}

	fn increment_index_by(&self, by: usize) {
		self.index.set(self.index.get() + by);
	}

	fn get_current_character(&self) -> char {
		self.text.chars().nth(self.index.get()).unwrap()
	}

	fn get_current_character_offset(&self, offset: usize) -> char {
		self.text.chars().nth(self.index.get() + offset).unwrap()
	}

	fn lex_white_space(&self) -> Option<Token<'a>> {
		loop {
			match self.text.chars().nth(self.index.get()) {
				Some(current_character) => {
					if current_character.is_whitespace() {
						self.increment_index();
					} else {
						return None;
					}
				},
				None => {
					return Some(Token::new(self.text.len(), "\0", TokenKind::End));
				}
			}
		}
	}

	fn lex_operator(&self) -> Option<Token<'a>> {
		let current_index = self.index.get();
		let arithmetic_operator_token_kind = match self.get_current_character() {
			'+' => Some(TokenKind::PlusOperator),
			'-' => Some(TokenKind::MinusOperator),
			'*' => Some(TokenKind::StarOperator),
			'/' => Some(TokenKind::SlashOperator),
			'%' => Some(TokenKind::PercentageOperator),
			'(' => Some(TokenKind::OpenParenthesis),
			')' => Some(TokenKind::CloseParenthesis),
			'!' => {
				if self.get_current_character_offset(1) == '=' {
					self.increment_index_by(2);
					return Some(Token::new(current_index, &self.text[current_index..self.index.get()], TokenKind::NotEqualityOperator));
				} else {
					Some(TokenKind::LogicalNotOperator)
				}
			},
			'|' => {
				if self.get_current_character_offset(1) == '|' {
					self.increment_index_by(2);
					return Some(Token::new(current_index, &self.text[current_index..self.index.get()], TokenKind::LogicalOrOperator));
				} else {
					None
				}
			},
			'&' => {
				if self.get_current_character_offset(1) == '&' {
					self.increment_index_by(2);
					return Some(Token::new(current_index, &self.text[current_index..self.index.get()], TokenKind::LogicalAndOperator));
				} else {
					None
				}
			},
			'<' => {
				if self.get_current_character_offset(1) == '=' {
					self.increment_index_by(2);
					return Some(Token::new(current_index, &self.text[current_index..self.index.get()], TokenKind::LessThanEqualToOperator));
				} else {
					Some(TokenKind::LessThanOperator)
				}
			},
			'>' => {
				if self.get_current_character_offset(1) == '=' {
					self.increment_index_by(2);
					return Some(Token::new(current_index, &self.text[current_index..self.index.get()], TokenKind::GreaterThanEqualToOperator));
				} else {
					Some(TokenKind::GreaterThanOperator)
				}
			},
			'=' => {
				if self.get_current_character_offset(1) == '=' {
					self.increment_index_by(2);
					return Some(Token::new(current_index, &self.text[current_index..self.index.get()], TokenKind::EqualityOperator));
				} else {
					Some(TokenKind::AssignmentOperator)
				}
			},
			_ => None
		};

		if let Some(arithmetic_operator_token_kind) = arithmetic_operator_token_kind {
			self.increment_index();
			Some(Token::new(current_index, &self.text[current_index..self.index.get()], arithmetic_operator_token_kind))
		} else {
			None
		}
	}

	fn lex_number_literal(&self) -> Option<Token<'a>> {
		if self.get_current_character().is_numeric() {
			let start_position = self.index.get();
			self.increment_index();
			let mut number_literal_kind = TokenKind::IntegerLiteral;

			loop {
				let current_character = self.get_current_character();

				if current_character == '.' {
					if number_literal_kind == TokenKind::FloatingPointLiteral {
						return None;
					} else {
						number_literal_kind = TokenKind::FloatingPointLiteral;
					}
				} else if !current_character.is_numeric() {
					break;
				}

				self.increment_index();

				if self.text.len() <= self.index.get() {
					break;
				}
			}

			Some(Token::new(start_position, &self.text[start_position..self.index.get()], number_literal_kind))
		} else {
			None
		}
	}

	fn lex_string_literal(&self) -> Option<Token<'a>> {
		if self.get_current_character() == '"' {
			let start_position = self.index.get();
			self.increment_index();

			while self.get_current_character() != '"' {
				self.increment_index();

				if self.text.len() <= self.index.get() {
					return None;
				}
			}

			self.increment_index();

			Some(Token::new(start_position, &self.text[start_position..self.index.get()], TokenKind::StringLiteral))
		} else {
			None
		}
	}

	fn lex_character_literal(&self) -> Option<Token<'a>> {
		if self.get_current_character() == '\'' && self.get_current_character_offset(2) == '\'' {
			let current_index = self.index.get();
			self.increment_index_by(3);

			Some(Token::new(current_index, &self.text[current_index..self.index.get()], TokenKind::CharacterLiteral))
		} else {
			None
		}
	}

	fn lex_keyword_and_identifier(&self) -> Option<Token<'a>> {
		if self.get_current_character().is_alphabetic() {
			let start_position = self.index.get();
			self.increment_index();

			while self.get_current_character().is_alphanumeric() {
				self.increment_index();

				if self.text.len() <= self.index.get() {
					break;
				}
			}

			Some(Token::new(start_position, &self.text[start_position..self.index.get()], TokenKind::Identifier))
		} else {
			None
		}
	}
}
