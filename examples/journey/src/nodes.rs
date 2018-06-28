use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Node {
    Cat(Box<Node>, Box<Node>),
    Star(String),
    Symbol(String),
    Literal(String),
    Slash,
    Dot,
    Or,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Node::*;

        match self {
            Slash => write!(f, "/"),
            Dot => write!(f, "."),
            Or => write!(f, "|"),
            Literal(string) => write!(f, "{}", string),
            Symbol(symbol) => write!(f, "{}", symbol),
            Star(star) => write!(f, "{}", star),
            Cat(lhs, rhs) => {
                write!(f, "{}", lhs)?;
                write!(f, "{}", rhs)?;
                Ok(())
            }
        }
    }
}

pub fn trace<T: fmt::Debug>(node: T) -> T {
    println!("{:?}", node);
    node
}

#[test]
fn recursion() {
    let cat1 = Node::Cat(Box::new(Node::Slash), Box::new(Node::Symbol("hello world".to_string())));

    let cat2 = Node::Cat(Box::new(Node::Slash), Box::new(cat1));
    let cat3 = Node::Cat(Box::new(Node::Slash), Box::new(cat2));
}
