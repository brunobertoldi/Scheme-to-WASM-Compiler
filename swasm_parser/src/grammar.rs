include!(concat!(env!("OUT_DIR"), "/grammar.rs"));

macro_rules! test_parse {
    (
        $( $expected:expr => $source:expr ),*,
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

    #[test]
    fn test_happy_path() {
        test_parse! {
            Node::Quote => "'",
            Node::Quote => "quote",
            Node::If => "if",

            Node::Bool(true) => "#t",
            Node::Bool(false) => "#f",

            Node::Number(Number::Exact(12)) => "12",
            Node::Number(Number::Inexact(0.5)) => "0.5",

            strnode("") => "\"\"",
            strnode("abc") => "\"abc\"",
            strnode(r#"abc"\"#) => r#""abc\"\\""#,

            idnode("id") => "id",
        };
    }
}
