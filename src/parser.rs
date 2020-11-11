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

impl Parser {
	pub fn new(mut tokens: Vec<Token>) -> Self {
		let env: EnvParser = EnvParser {};
		let ctoken: Token = tokens.remove(0);

		Parser {
			env,
			ctoken,
			tokens,
		}
	}

	fn advance(&mut self) {
		if self.tokens.len() > 0 {
			self.ctoken = self.tokens.remove(0);
		}
	}
}
