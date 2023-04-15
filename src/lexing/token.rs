#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
	IntegerLiteral,
	FloatingPointLiteral,
	PlusOperator,
	MinusOperator,
	SlashOperator,
	StarOperator,
	PercentageOperator,
	EqualsOperator,
	EqualityOperator,
	LessThanOperator,
	GreaterThanOperator,
	LessThanEqualOperator,
	GreaterThanEqualOperator,
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
