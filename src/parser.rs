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
mod tests {

	use super::{EnvParser, Parser};
	use crate::ast::{Expression, Statement};
	use crate::lexer::Lexer;
	use crate::token::Token;

	fn make_parser(source: &str) -> Parser {
		let mut lexer: Lexer = Lexer::new(source.to_string());
		let tokens: Vec<Token> = lexer.run().unwrap();
		Parser::new(tokens)
	}

	#[test]
	fn it_works() {
		assert_eq!(
			make_parser(""),
			Parser {
				env: EnvParser {},
				ctoken: Token::Eof,
				tokens: vec![],
			}
		);
	}

	#[test]
	fn source_empty() {
		assert_eq!(make_parser("").run().unwrap(), vec![]);
	}

	#[test]
	fn source_spaces() {
		assert_eq!(make_parser(" ").run().unwrap(), vec![]);
		assert_eq!(make_parser("\r").run().unwrap(), vec![]);
		assert_eq!(make_parser("\n").run().unwrap(), vec![]);
		assert_eq!(make_parser("\t").run().unwrap(), vec![]);

		assert_eq!(make_parser("\r\n\t ").run().unwrap(), vec![]);
	}

	#[test]
	fn source_punctuations() {
		fn punctuations() {
			assert_eq!(make_parser(";").run().is_err(), true);
		}

		fn with_spaces() {
			assert_eq!(make_parser("; \n;\r;\t;").run().is_err(), true);
		}

		fn with_integers() {
			assert_eq!(
				make_parser("1;35;57;87;").run().unwrap(),
				vec![
					Statement::Expression(Expression::Integer(1)),
					Statement::Expression(Expression::Integer(35)),
					Statement::Expression(Expression::Integer(57)),
					Statement::Expression(Expression::Integer(87)),
				]
			);
		}

		punctuations();
		with_spaces();
		with_integers();
	}

	#[test]
	fn source_integers() {
		assert_eq!(
			make_parser("012").run().unwrap(),
			vec![Statement::Expression(Expression::Integer(12))]
		);

		assert_eq!(
			make_parser(" 34\n45\n;94\r;48\t; 35 ;53 ").run().unwrap(),
			vec![
				Statement::Expression(Expression::Integer(34)),
				Statement::Expression(Expression::Integer(45)),
				Statement::Expression(Expression::Integer(94)),
				Statement::Expression(Expression::Integer(48)),
				Statement::Expression(Expression::Integer(35)),
				Statement::Expression(Expression::Integer(53)),
			]
		);

		assert_eq!(
			make_parser("05;67;").run().unwrap(),
			vec![
				Statement::Expression(Expression::Integer(5)),
				Statement::Expression(Expression::Integer(67)),
			]
		);
	}
}
