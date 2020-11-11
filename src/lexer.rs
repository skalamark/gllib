use crate::token::{string_is_token, Token, DIGITS, PUNCTUATIONS, SPACES};

#[derive(Debug, PartialEq)]
struct EnvLexer {}

#[derive(Debug, PartialEq)]
pub struct Lexer {
	env: EnvLexer,
	cchar: String,
	chars: Vec<char>,
	textlines: Vec<String>,
}

impl Lexer {
	pub fn new(source: String) -> Self {
		let env: EnvLexer = EnvLexer {};
		let mut textlines: Vec<String> = source.lines().map(|l: &str| format!("{}\n", l)).collect();
		if textlines.len() == 0 {
			textlines.push(source);
		}
		let mut chars: Vec<char> = textlines.remove(0).chars().map(|c: char| c).collect();
		let cchar: String = if chars.len() > 0 {
			chars.remove(0).to_string()
		} else {
			String::new()
		};

		Lexer {
			env,
			cchar,
			chars,
			textlines,
		}
	}

	fn init_fields(&mut self) {
		while self.cchar.is_empty() {
			if self.chars.len() == 0 && self.textlines.len() == 0 {
				break;
			}
			self.advance();
		}
	}

	fn advance(&mut self) {
		self.advance_char();
	}

	fn advance_char(&mut self) {
		if self.chars.len() > 0 {
			self.cchar = self.chars.remove(0).to_string();
		} else {
			self.advance_textline()
		}
	}

	fn advance_textline(&mut self) {
		if self.textlines.len() > 0 {
			let textline = self.textlines.remove(0);
			if textline.is_empty() {
				self.advance_textline();
			} else {
				self.chars = textline.chars().map(|c: char| c).collect();
				self.advance_char();
			}
		} else {
			if self.chars.len() == 0 {
				self.cchar = String::new();
			}
		}
	}

	fn collision_check(&self) -> Result<(), String> {
		if self.cchar.is_empty()
			|| SPACES.contains(&self.cchar.as_str())
			|| PUNCTUATIONS.contains(&self.cchar.as_str())
		{
			Ok(())
		} else {
			if string_is_token(self.cchar.clone()) {
				Err(format!("SyntaxError: invalid syntax"))
			} else {
				Err(format!("SyntaxError: invalid character in identifier"))
			}
		}
	}

	fn make_token(&mut self) -> Result<Token, String> {
		if self.cchar.is_empty() {
			Ok(Token::Eof)
		} else if SPACES.contains(&self.cchar.as_str()) {
			if self.cchar == "\n" {
				self.advance();
				Ok(Token::NewLine)
			} else {
				self.advance();
				self.make_token()
			}
		} else if PUNCTUATIONS.contains(&self.cchar.as_str()) {
			if self.cchar == ";" {
				self.advance();
				return Ok(Token::SemiColon);
			}

			self.make_token()
		} else if DIGITS.contains(&self.cchar.as_str()) {
			let mut number_literal: String = String::new();

			while self.cchar.is_empty() == false && DIGITS.contains(&self.cchar.as_str()) {
				number_literal.push_str(&self.cchar.as_str());
				self.advance();
			}

			match self.collision_check() {
				Ok(_) => {}
				Err(details) => return Err(details),
			}

			Ok(Token::Integer(number_literal))
		} else {
			Err(format!("SyntaxError: invalid character in identifier"))
		}
	}

	pub fn run(&mut self) -> Result<Vec<Token>, String> {
		let mut tokens: Vec<Token> = Vec::new();
		self.init_fields();
		loop {
			match self.make_token() {
				Ok(token) => {
					let is_eof: bool = token.is_eof();
					tokens.push(token);
					if is_eof {
						break;
					}
				}
				Err(details) => return Err(details),
			}
		}
		Ok(tokens)
	}
}

#[cfg(test)]
mod tests {

	use super::{EnvLexer, Lexer};
	use crate::token::Token::*;

	fn make_lexer(source: &str) -> Lexer {
		Lexer::new(source.to_string())
	}

	#[test]
	fn it_works() {
		assert_eq!(
			make_lexer(""),
			Lexer {
				env: EnvLexer {},
				cchar: "".to_string(),
				chars: vec![],
				textlines: vec![]
			}
		);
	}

	#[test]
	fn source_empty() {
		assert_eq!(make_lexer("").run().unwrap(), vec![Eof]);
	}

	#[test]
	fn source_spaces() {
		assert_eq!(make_lexer(" ").run().unwrap(), vec![NewLine, Eof]);
		assert_eq!(make_lexer("\r").run().unwrap(), vec![NewLine, Eof]);
		assert_eq!(make_lexer("\n").run().unwrap(), vec![NewLine, Eof]);
		assert_eq!(make_lexer("\t").run().unwrap(), vec![NewLine, Eof]);

		assert_eq!(
			make_lexer("\r\n\t ").run().unwrap(),
			vec![NewLine, NewLine, Eof]
		);
	}

	#[test]
	fn source_punctuations() {
		fn punctuations() {
			assert_eq!(
				make_lexer(";").run().unwrap(),
				vec![SemiColon, NewLine, Eof]
			);
		}

		fn with_spaces() {
			assert_eq!(
				make_lexer("; \n;\r;\t;").run().unwrap(),
				vec![SemiColon, NewLine, SemiColon, SemiColon, SemiColon, NewLine, Eof]
			);
		}

		fn with_integers() {
			assert_eq!(
				make_lexer(";1;435;3457;987;").run().unwrap(),
				vec![
					SemiColon,
					Integer(String::from("1")),
					SemiColon,
					Integer(String::from("435")),
					SemiColon,
					Integer(String::from("3457")),
					SemiColon,
					Integer(String::from("987")),
					SemiColon,
					NewLine,
					Eof
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
			make_lexer("0123456789").run().unwrap(),
			vec![Integer(String::from("0123456789")), NewLine, Eof]
		);

		assert_eq!(
			make_lexer(" 834\n23894\r3948\t98735 345 ").run().unwrap(),
			vec![
				Integer(String::from("834")),
				NewLine,
				Integer(String::from("23894")),
				Integer(String::from("3948")),
				Integer(String::from("98735")),
				Integer(String::from("345")),
				NewLine,
				Eof
			]
		);

		assert_eq!(
			make_lexer(";012345;6789;").run().unwrap(),
			vec![
				SemiColon,
				Integer(String::from("012345")),
				SemiColon,
				Integer(String::from("6789")),
				SemiColon,
				NewLine,
				Eof
			]
		);
	}
}
