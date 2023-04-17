pub enum SyntaxKind {
	Expression
}

pub trait Syntax {
	fn get_syntax_kind(&self) -> SyntaxKind;
	fn print(&self, indentation: usize);
}

pub struct SyntaxRoot {
	root: Box<dyn Syntax>
}

impl SyntaxRoot {
	pub fn new(root: Box<dyn Syntax>) -> Self {
		Self {
			root
		}
	}

	pub fn print(&self) {
		self.root.print(0);
	}
}
