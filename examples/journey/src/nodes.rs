use scanner;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    Token(scanner::Token),
    Group(Vec<Expression>),
}
