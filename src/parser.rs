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

	fn remove_newlines(&mut self) {
		loop {
			if self.ctoken.is_newline() == false {
				break;
			}
			self.advance();
		}
	}

	fn make_atom(&mut self) -> Result<Expression, String> {
		self.remove_newlines();
		match self.ctoken.clone() {
			Token::Integer(number) => {
				let number_parse = number.parse::<u8>();
				if number_parse.is_err() {
					return Err(format!("RuntimeError: Overflow"));
				}
				self.advance();
				Ok(Expression::Integer(number_parse.unwrap()))
			}
			_ => return Err(format!("SyntaxError: invalid syntax")),
		}
	}

	fn make_expression(&mut self) -> Result<Expression, String> {
		self.remove_newlines();
		let expression: Expression = match self.make_atom() {
			Ok(atom_expression) => atom_expression,
			Err(details) => return Err(details),
		};

		Ok(expression)
	}

	fn make_statement(&mut self) -> Result<Statement, String> {
		self.remove_newlines();
		let statement: Statement = match self.ctoken.clone() {
			_ => match self.make_expression() {
				Ok(expression) => Statement::Expression(expression),
				Err(details) => return Err(details),
			},
		};

		let mut last_ctoken_is_newline: bool = false;
		if self.ctoken.is_newline() {
			last_ctoken_is_newline = true;
			self.advance();
		}

		if self.ctoken.is_semicolon() == false && last_ctoken_is_newline == false {
			return Err(format!("SyntaxError: invalid syntax"));
		}

		self.remove_newlines();
		if self.ctoken.is_semicolon() {
			self.advance();
		}

		Ok(statement)
	}

	pub fn run(&mut self) -> Result<Vec<Statement>, String> {
		let mut ast: Vec<Statement> = Vec::new();
		loop {
			self.remove_newlines();
			if self.ctoken.is_eof() {
				break;
			}

			match self.make_statement() {
				Ok(statement) => {
					ast.push(statement);
				}
				Err(details) => return Err(details),
			}
		}
		Ok(ast)
	}
}

#[cfg(test)]
mod tests {}
