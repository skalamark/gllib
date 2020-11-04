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
