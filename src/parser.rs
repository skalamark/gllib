use crate::ast::{Expression, Statement};
use crate::token::Token;

#[derive(Debug, PartialEq)]
struct EnvParser {}

#[derive(Debug, PartialEq)]
pub struct Parser {
	env: EnvParser,
	ctoken: Token,
	tokens: Vec<Token>,
}
