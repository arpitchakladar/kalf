#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
// Literals
	StringLiteral,
	CharacterLiteral,
	IntegerLiteral,
	FloatingPointLiteral,

// Arithmetic
	PlusOperator,
	MinusOperator,
	SlashOperator,
	StarOperator,
	PercentageOperator,

// Assignment
	AssignmentOperator,

// Relational
	EqualityOperator,
	NotEqualityOperator,
	LessThanOperator,
	GreaterThanOperator,
	LessThanEqualToOperator,
	GreaterThanEqualToOperator,

// Logical
	LogicalNotOperator,
	LogicalAndOperator,
	LogicalOrOperator,

// Separator
	OpenParenthesis,
	CloseParenthesis,

	Identifier,
	Keyword,

	End
}

pub struct Token<'a> {
	index: usize,
	text: &'a str,
	kind: TokenKind
}

impl<'a> Token<'a> {
	pub fn new(index: usize, text: &'a str, kind: TokenKind) -> Self {
		Self {
			index,
			text,
			kind
		}
	}

	pub fn text(&self) -> &'a str {
		self.text
	}

	pub fn kind(&self) -> TokenKind {
		self.kind
	}
}
