pub const SPACES: &str = "\r\n\t ";
pub const DIGITS: &str = "0123456789";
pub const PUNCTUATIONS: &str = ";";

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
	Eof,
	SemiColon,
	NewLine,
	Integer(String),
}

impl Token {
	pub fn is_eof(&self) -> bool {
		match self {
			Token::Eof => true,
			_ => false,
		}
	}

	pub fn is_semicolon(&self) -> bool {
		match self {
			Token::SemiColon => true,
			_ => false,
		}
	}
}

pub fn string_is_token(character: String) -> bool {
	if SPACES.contains(&character.as_str()) {
		true
	} else {
		false
	}
}
