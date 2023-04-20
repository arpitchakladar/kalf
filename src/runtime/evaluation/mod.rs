mod expression;

use expression::evaluate_expression;
use crate::syntax::Syntax;

pub fn evaluate_syntax(syntax: &Syntax) -> f64 {
	match syntax {
		Syntax::Expression(expression) => evaluate_expression(expression)
	}
}
