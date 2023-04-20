use std::rc::Rc;
use crate::syntax::{
	Expression
};

pub struct ParenthesisedExpression<'a> {
	content: Rc<Expression<'a>>
}

impl<'a> ParenthesisedExpression<'a> {
	pub fn new(content: Rc<Expression<'a>>) -> Self {
		Self {
			content
		}
	}

	pub fn content(&self) -> &Expression<'a> {
		&self.content
	}
}
