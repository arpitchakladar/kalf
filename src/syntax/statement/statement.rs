use crate::syntax::{
	VariableDeclarationStatement
};

pub enum Statement<'a> {
	VariableDeclaration(VariableDeclarationStatement<'a>),
}
