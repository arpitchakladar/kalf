#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug)]
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

	pub fn get_kind(&self) -> TokenKind {
		self.kind
	}
}
