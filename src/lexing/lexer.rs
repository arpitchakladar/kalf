use super::{
	Token,
	TokenKind
};

pub struct Lexer<'a> {
	index: usize,
	text: &'a str
}

impl<'a> Lexer<'a> {
	pub fn new(text: &'a str) -> Self {
		Self {
			index: 0,
			text
		}
	}

	pub fn lex(&mut self) -> Result<Token<'a>, &'static str> {
		if let Some(end_token) = self.lex_white_space() {
			return Ok(end_token);
		}

		if let Some(separator_token) = self.lex_separator() {
			return Ok(separator_token);
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

	fn get_current_character(&self) -> char {
		self.text.chars().nth(self.index).unwrap()
	}

	fn get_current_character_offset(&self, offset: usize) -> char {
		self.text.chars().nth(self.index + offset).unwrap()
	}

	fn lex_white_space(&mut self) -> Option<Token<'a>> {
		loop {
			match self.text.chars().nth(self.index) {
				Some(current_character) => {
					if current_character.is_whitespace() {
						self.index += 1;
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

	fn lex_separator(&mut self) -> Option<Token<'a>> {
		let current_index = self.index;
		let separator_token_kind = match self.get_current_character() {
			'(' => Some(TokenKind::OpenParenthesis),
			')' => Some(TokenKind::CloseParenthesis),
			_ => None
		};

		if let Some(separator_token_kind) = separator_token_kind {
			self.index += 1;
			Some(Token::new(current_index, &self.text[current_index..self.index], separator_token_kind))
		} else {
			None
		}
	}

	fn lex_operator(&mut self) -> Option<Token<'a>> {
		let current_index = self.index;
		let arithmetic_operator_token_kind = match self.get_current_character() {
			'+' => Some(TokenKind::PlusOperator),
			'-' => Some(TokenKind::MinusOperator),
			'*' => Some(TokenKind::StarOperator),
			'/' => Some(TokenKind::SlashOperator),
			'%' => Some(TokenKind::PercentageOperator),
			'=' => {
				if self.get_current_character_offset(1) == '=' {
					self.index += 2;
					return Some(Token::new(current_index, &self.text[current_index..self.index], TokenKind::EqualityOperator));
				} else {
					Some(TokenKind::AssignmentOperator)
				}
			},
			_ => None
		};

		if let Some(arithmetic_operator_token_kind) = arithmetic_operator_token_kind {
			self.index += 1;
			Some(Token::new(current_index, &self.text[current_index..self.index], arithmetic_operator_token_kind))
		} else {
			None
		}
	}

	fn lex_number_literal(&mut self) -> Option<Token<'a>> {
		if self.get_current_character().is_numeric() {
			let start_position = self.index;
			self.index += 1;
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

				self.index += 1;

				if self.text.len() <= self.index {
					break;
				}
			}

			Some(Token::new(start_position, &self.text[start_position..self.index], number_literal_kind))
		} else {
			None
		}
	}

	fn lex_string_literal(&mut self) -> Option<Token<'a>> {
		if self.get_current_character() == '"' {
			let start_position = self.index;
			self.index += 1;

			while self.get_current_character() != '"' {
				self.index += 1;

				if self.text.len() <= self.index {
					return None;
				}
			}

			self.index += 1;

			Some(Token::new(start_position, &self.text[start_position..self.index], TokenKind::StringLiteral))
		} else {
			None
		}
	}

	fn lex_character_literal(&mut self) -> Option<Token<'a>> {
		if self.get_current_character() == '\'' && self.get_current_character_offset(2) == '\'' {
			let current_index = self.index;
			self.index += 3;

			Some(Token::new(current_index, &self.text[current_index..self.index], TokenKind::CharacterLiteral))
		} else {
			None
		}
	}

	fn lex_keyword_and_identifier(&mut self) -> Option<Token<'a>> {
		if self.get_current_character().is_alphabetic() {
			let start_position = self.index;
			self.index += 1;

			while self.get_current_character().is_alphanumeric() {
				self.index += 1;

				if self.text.len() <= self.index {
					break;
				}
			}

			Some(Token::new(start_position, &self.text[start_position..self.index], TokenKind::Identifier))
		} else {
			None
		}
	}
}
