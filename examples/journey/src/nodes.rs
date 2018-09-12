use scanner;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    Token(scanner::Token),
    Group(Vec<Expression>),
    Or(Vec<Expression>),
}

pub fn concat<T: Clone>(lhs: T, rhs: &[T]) -> Vec<T> {
    let mut v = Vec::with_capacity(1 + rhs.len());
    v.push(lhs);
    v.extend_from_slice(rhs);
    v
}
