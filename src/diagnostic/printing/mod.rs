mod expression;

use crate::syntax::Syntax;
use expression::print_expression;

pub fn print_syntax(syntax: &Syntax) {
	match syntax {
		Syntax::Expression(expression) => print_expression(expression, 0)
	}
}
