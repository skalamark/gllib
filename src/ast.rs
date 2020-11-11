#[derive(Debug, PartialEq)]
pub enum Statement {
	Expression(Expression),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
	Integer(u8),
}
