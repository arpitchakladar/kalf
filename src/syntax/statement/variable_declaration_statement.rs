use crate::types::Type;

pub struct VariableDeclarationStatement<'a> {
	type: &'a Type,
	identifier: &'a Token<'a>,
	value: Rc<Expression<'a>>
}
