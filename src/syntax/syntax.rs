use crate::syntax::Expression;

#[derive(Clone)]
pub enum Syntax<'a> {
	Expression(Expression<'a>)
}
