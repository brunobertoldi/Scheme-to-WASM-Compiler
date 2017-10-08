include!(concat!(env!("OUT_DIR"), "/grammar.rs"));

macro_rules! test_parse {
    (
        $( $source:expr => $expected:expr ),*,
    ) => {
        $(
            assert_eq!($expected, parse_Node($source).unwrap());
        )*
    };
}

#[cfg(test)]
mod tests {
    use super::parse_Node;
    use ast::{Node, Number};

    fn strnode<S: Into<String>>(s: S) -> Node { Node::String(s.into()) }
    fn idnode<S: Into<String>>(s: S) -> Node { Node::Ident(s.into()) }
    fn exact(i: i64) -> Node { Node::Number(Number::Exact(i)) }
    fn inexact(f: f64) -> Node { Node::Number(Number::Inexact(f)) }

    #[test]
    fn test_node_happy_path() {
        test_parse! {
            "'" => Node::Quote,
            "quote" => Node::Quote,
            "if" => Node::If,

            "#t" => Node::Bool(true),
            "#f" => Node::Bool(false),

            "12" => exact(12),
            "+12" => exact(12),
            "-12" => exact(-12),
            "0.5" => inexact(0.5),
            "+0.5" => inexact(0.5),
            "-0.5" => inexact(-0.5),

            "\"\"" => strnode(""),
            "\"abc\"" => strnode("abc"),
            r#""abc\"\\""# => strnode(r#"abc"\"#),

            "id" => idnode("id"),
            "LoWeR" => idnode("lower"),
            "$" => idnode("$"),
            "a1" => idnode("a1"),
            "+" => idnode("+"),
            "-" => idnode("-"),
            "..." => idnode("..."),

            "()" => Node::List(Vec::new()),
            "(#t 12 a)" => Node::List(vec![Node::Bool(true), exact(12), idnode("a")]),

            "(a . b)" => Node::DottedList(vec![idnode("a")], Box::new(idnode("b"))),
        };
    }
}
