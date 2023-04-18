use crate::syntax::{
	Expression
};

pub struct ParenthesisedExpression<'a> {
	content: Box<Expression<'a>>
}

impl<'a> ParenthesisedExpression<'a> {
	pub fn new(content: Box<Expression<'a>>) -> Self {
		Self {
			content
		}
	}

	pub fn get_content(&self) -> &Expression<'a> {
		&self.content
	}
}
