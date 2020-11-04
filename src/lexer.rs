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
}
