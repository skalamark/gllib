pub const SPACES: &str = "\r\n\t ";

#[derive(Debug, PartialEq)]
pub enum Token {
	Eof,
}

impl Token {
	pub fn is_eof(&self) -> bool {
		match self {
			Token::Eof => true,
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
