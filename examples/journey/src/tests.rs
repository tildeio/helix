use parser::ExpressionsParser;
use nodes::Node;

#[test]
fn parse_star() {
    let parser = ExpressionsParser::new();
    let result = parser.parse("*foo");

    assert_eq!(result.unwrap(), Node::Star("*foo".to_string()));
}

#[test]
fn parse_path() {
    let parser = ExpressionsParser::new();
    let result = parser.parse("/literal/:symbol/*star");

    assert_eq!(result.clone().unwrap(), Node::Cat(
        Box::new(Node::Slash),
        Box::new(Node::Cat(
            Box::new(Node::Literal("literal".to_string())),
            Box::new(Node::Cat(
                Box::new(Node::Slash),
                Box::new(Node::Cat(
                    Box::new(Node::Symbol(":symbol".to_string())),
                    Box::new(Node::Cat(
                        Box::new(Node::Slash),
                        Box::new(Node::Star("*star".to_string()))
                    ))
                ))
            ))
        ))
    ));

    assert_eq!(result.unwrap().to_string(), "/literal/:symbol/*star");
}
