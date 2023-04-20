use crate::syntax::Expression;

pub enum Syntax<'a> {
	Expression(Expression<'a>)
}
