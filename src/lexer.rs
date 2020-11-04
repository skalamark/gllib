use crate::token::{Token, SPACES};

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
		let mut textlines: Vec<String> = source.lines().map(|l: &str| l.to_string()).collect();
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

	fn make_token(&mut self) -> Result<Token, String> {
		if self.cchar.is_empty() {
			Ok(Token::Eof)
		} else if SPACES.contains(&self.cchar.as_str()) {
			self.advance();
			self.make_token()
		} else {
			Err(format!("SyntaxError: invalid character in identifier"))
		}
	}
}
