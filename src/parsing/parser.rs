use std::rc::Rc;
use std::cell::Cell;
use crate::syntax::Syntax;
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

	pub(in crate::parsing) fn increment_index(&self) {
		self.index.set(self.index.get() + 1);
	}

	pub(in crate::parsing) fn current_token(&self) -> &Token<'a> {
		if self.index.get() < self.tokens.len() {
			&self.tokens[self.index.get()]
		} else {
			&self.tokens[self.tokens.len() - 1]
		}
	}

	pub fn parse(&self) -> Rc<Syntax<'_>> {
		Rc::new(Syntax::Expression(self.parse_expression()))
	}
}
