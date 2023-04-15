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
		if self.index >= self.text.len() {
			return Ok(Token::new(self.text.len(), "\0", TokenKind::End));
		}

		let mut current_character = self.text.chars().nth(self.index).unwrap();

		while current_character.is_whitespace() {
			self.index += 1;
			current_character = self.text.chars().nth(self.index).unwrap();
		}

		if current_character.is_numeric() {
			let start_position = self.index;
			self.index += 1;
			let mut number_literal_kind = TokenKind::IntegerLiteral;
			let mut current_character = self.text.chars().nth(self.index).unwrap();

			while current_character.is_numeric() || current_character == '.' {
				if current_character == '.' {
					if number_literal_kind == TokenKind::FloatingPointLiteral {
						return Err("Invalid floating point number literal.");
					} else {
						number_literal_kind = TokenKind::FloatingPointLiteral;
					}
				}

				self.index += 1;

				if self.text.len() <= self.index {
					break;
				} else {
					current_character = self.text.chars().nth(self.index).unwrap();
				}
			}

			return Ok(Token::new(start_position, &self.text[start_position..self.index], number_literal_kind));
		}

		if current_character.is_alphabetic() {
			let start_position = self.index;
			self.index += 1;

			while self.text.chars().nth(self.index).unwrap().is_alphanumeric() {
				self.index += 1;

				if self.text.len() <= self.index {
					break;
				}
			}

			return Ok(Token::new(start_position, &self.text[start_position..self.index], TokenKind::Identifier));
		}

		let current_index = self.index;
		self.index += 1;
		match current_character {
			'+' => Ok(Token::new(current_index, &self.text[current_index..self.index], TokenKind::PlusOperator)),
			'-' => Ok(Token::new(current_index, &self.text[current_index..self.index], TokenKind::MinusOperator)),
			'*' => Ok(Token::new(current_index, &self.text[current_index..self.index], TokenKind::StarOperator)),
			'/' => Ok(Token::new(current_index, &self.text[current_index..self.index], TokenKind::SlashOperator)),
			'%' => Ok(Token::new(current_index, &self.text[current_index..self.index], TokenKind::PercentageOperator)),
			'=' => {
				if self.text.chars().nth(self.index).unwrap() == '=' {
					self.index += 1;
					Ok(Token::new(current_index, &self.text[current_index..self.index], TokenKind::EqualityOperator))
				} else {
					Ok(Token::new(current_index, &self.text[current_index..self.index], TokenKind::EqualsOperator))
				}
			},
			_ => Err("Unknown token.")
		}
	}
}
