pub enum SyntaxKind {
	Expression
}

pub trait Syntax {
	fn get_syntax_kind(&self) -> SyntaxKind;
}
