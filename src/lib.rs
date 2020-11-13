pub mod ast;
pub mod lexer;
pub mod object;
pub mod parser;
pub mod token;

#[cfg(test)]
mod tests {
	use super::lexer::Lexer;
	use super::parser::Parser;
	use super::token::Token;
	use crate::ast::Statement;

	#[test]
	fn basic_lexer() {
		// Source
		let source: &str = "";
		// Create a Lexer
		let mut lexer: Lexer = Lexer::new(source.to_string());
		// Run the Lexer
		let result_lexer: Result<Vec<Token>, String> = lexer.run();
		// The Lexer have a Err
		if result_lexer.is_err() {
			return;
		}
		// Get Vec of Token from Lexer
		let tokens: Vec<Token> = result_lexer.unwrap();
		assert_eq!(tokens, vec![Token::Eof]);
	}

	#[test]
	fn basic_parser() {
		// Vec of Token
		let tokens: Vec<Token> = vec![Token::Eof];
		// Create a Parser
		let mut parser: Parser = Parser::new(tokens);
		// Run the Parser
		let result_parser: Result<Vec<Statement>, String> = parser.run();
		// The Parser have a Err
		if result_parser.is_err() {
			return;
		}
		// Get the ast from Parser
		let ast: Vec<Statement> = result_parser.unwrap();
		assert_eq!(ast, vec![]);
	}
}
