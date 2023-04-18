pub enum Token<'a> {
// Literals
	StringLiteral(TokenContent<'a>),
	CharacterLiteral(TokenContent<'a>),
	IntegerLiteral(TokenContent<'a>),
	FloatingPointLiteral(TokenContent<'a>),

// Arithmetic
	PlusOperator(TokenContent<'a>),
	MinusOperator(TokenContent<'a>),
	SlashOperator(TokenContent<'a>),
	StarOperator(TokenContent<'a>),
	PercentageOperator(TokenContent<'a>),

// Assignment
	AssignmentOperator(TokenContent<'a>),

// Relational
	EqualityOperator(TokenContent<'a>),
	NotEqualityOperator(TokenContent<'a>),
	LessThanOperator(TokenContent<'a>),
	GreaterThanOperator(TokenContent<'a>),
	LessThanEqualToOperator(TokenContent<'a>),
	GreaterThanEqualToOperator(TokenContent<'a>),

// Logical
	LogicalNotOperator(TokenContent<'a>),
	LogicalAndOperator(TokenContent<'a>),
	LogicalOrOperator(TokenContent<'a>),

// Separator
	OpenParenthesis(TokenContent<'a>),
	CloseParenthesis(TokenContent<'a>),

	Identifier(TokenContent<'a>),
	Keyword(TokenContent<'a>),

	End
}

#[derive(Debug, Clone, Copy)]
pub struct TokenContent<'a> {
	index: usize,
	text: &'a str
}

impl<'a> TokenContent<'a> {
	pub fn new(index: usize, text: &'a str) -> Self {
		Self {
			index,
			text
		}
	}

	pub fn get_text(&self) -> &'a str {
		self.text
	}
}
