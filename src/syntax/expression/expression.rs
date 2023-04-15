use crate::lexing::{
	TokenKind,
	Token
};
use crate::syntax::{
	SyntaxKind,
	Syntax
};

pub enum ExpressionKind {
	Binary,
	Unary,
	Condition,
	Literal,
	Parenthesised
}

pub trait Expression: Syntax {
	fn get_expression_kind(&self) -> ExpressionKind;
}

pub fn get_expression_precendence(token: &Token) -> u8 {
	match token.get_kind() {
		TokenKind::PlusOperator | TokenKind::MinusOperator => 8,
		TokenKind::StarOperator | TokenKind::SlashOperator | TokenKind::PercentageOperator => 9,
		TokenKind::OpenParenthesis | TokenKind::CloseParenthesis => 11,
		_ => 0
	}
}
