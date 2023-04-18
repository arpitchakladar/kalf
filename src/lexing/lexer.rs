use std::cell::Cell;
use crate::lexing::{
	Token,
	TokenContent
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
					return Some(Token::End);
				}
			}
		}
	}

	fn lex_operator(&self) -> Option<Token<'a>> {
		let current_index = self.index.get();
		match self.get_current_character() {
			'+' => {
				self.increment_index();
				Some(Token::PlusOperator(TokenContent::new(current_index, &self.text[current_index..self.index.get()])))
			},
			'-' => {
				self.increment_index();
				Some(Token::MinusOperator(TokenContent::new(current_index, &self.text[current_index..self.index.get()])))
			},
			'*' => {
				self.increment_index();
				Some(Token::StarOperator(TokenContent::new(current_index, &self.text[current_index..self.index.get()])))
			},
			'/' => {
				self.increment_index();
				Some(Token::SlashOperator(TokenContent::new(current_index, &self.text[current_index..self.index.get()])))
			},
			'%' => {
				self.increment_index();
				Some(Token::PercentageOperator(TokenContent::new(current_index, &self.text[current_index..self.index.get()])))
			},
			'(' => {
				self.increment_index();
				Some(Token::OpenParenthesis(TokenContent::new(current_index, &self.text[current_index..self.index.get()])))
			},
			')' => {
				self.increment_index();
				Some(Token::CloseParenthesis(TokenContent::new(current_index, &self.text[current_index..self.index.get()])))
			},
			'!' => {
				if self.get_current_character_offset(1) == '=' {
					self.increment_index_by(2);
					Some(Token::NotEqualityOperator(TokenContent::new(current_index, &self.text[current_index..self.index.get()])))
				} else {
					self.increment_index();
					Some(Token::LogicalNotOperator(TokenContent::new(current_index, &self.text[current_index..self.index.get()])))
				}
			},
			'|' => {
				if self.get_current_character_offset(1) == '|' {
					self.increment_index_by(2);
					Some(Token::LogicalOrOperator(TokenContent::new(current_index, &self.text[current_index..self.index.get()])))
				} else {
					None
				}
			},
			'&' => {
				if self.get_current_character_offset(1) == '&' {
					self.increment_index_by(2);
					Some(Token::LogicalAndOperator(TokenContent::new(current_index, &self.text[current_index..self.index.get()])))
				} else {
					None
				}
			},
			'<' => {
				if self.get_current_character_offset(1) == '=' {
					self.increment_index_by(2);
					Some(Token::LessThanEqualToOperator(TokenContent::new(current_index, &self.text[current_index..self.index.get()])))
				} else {
					self.increment_index();
					Some(Token::LessThanOperator(TokenContent::new(current_index, &self.text[current_index..self.index.get()])))
				}
			},
			'>' => {
				if self.get_current_character_offset(1) == '=' {
					self.increment_index_by(2);
					Some(Token::GreaterThanEqualToOperator(TokenContent::new(current_index, &self.text[current_index..self.index.get()])))
				} else {
					self.increment_index();
					Some(Token::GreaterThanOperator(TokenContent::new(current_index, &self.text[current_index..self.index.get()])))
				}
			},
			'=' => {
				if self.get_current_character_offset(1) == '=' {
					self.increment_index_by(2);
					Some(Token::EqualityOperator(TokenContent::new(current_index, &self.text[current_index..self.index.get()])))
				} else {
					self.increment_index();
					Some(Token::AssignmentOperator(TokenContent::new(current_index, &self.text[current_index..self.index.get()])))
				}
			},
			_ => None
		}
	}

	fn lex_number_literal(&self) -> Option<Token<'a>> {
		if self.get_current_character().is_numeric() {
			let start_position = self.index.get();
			self.increment_index();
			let mut is_integer_literal = true;

			loop {
				let current_character = self.get_current_character();

				if current_character == '.' {
					if !is_integer_literal {
						return None;
					} else {
						is_integer_literal = false;
					}
				} else if !current_character.is_numeric() {
					break;
				}

				self.increment_index();

				if self.text.len() <= self.index.get() {
					break;
				}
			}

			Some(if is_integer_literal {
				Token::IntegerLiteral(TokenContent::new(start_position, &self.text[start_position..self.index.get()]))
			} else {
				Token::FloatingPointLiteral(TokenContent::new(start_position, &self.text[start_position..self.index.get()]))
			})
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

			Some(Token::StringLiteral(TokenContent::new(start_position, &self.text[start_position..self.index.get()])))
		} else {
			None
		}
	}

	fn lex_character_literal(&self) -> Option<Token<'a>> {
		if self.get_current_character() == '\'' && self.get_current_character_offset(2) == '\'' {
			let current_index = self.index.get();
			self.increment_index_by(3);

			Some(Token::CharacterLiteral(TokenContent::new(current_index, &self.text[current_index..self.index.get()])))
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

			Some(Token::Identifier(TokenContent::new(start_position, &self.text[start_position..self.index.get()])))
		} else {
			None
		}
	}
}
