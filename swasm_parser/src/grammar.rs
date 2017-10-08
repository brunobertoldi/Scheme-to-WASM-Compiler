include!(concat!(env!("OUT_DIR"), "/grammar.rs"));

#[cfg(test)]
mod tests {
    macro_rules! test_parse {
        (
            $( $source:expr => $expected:expr ),*,
        ) => {
            $(
                assert_eq!($expected, parse_SExpr($source).unwrap());
            )*
        };
    }

    macro_rules! test_err {
        (
            $( $source:expr ),*,
        ) => {
            $(
                parse_SExpr($source).unwrap_err();
            )*
        };
    }

    use super::parse_SExpr;
    use ast::{SExpr, Number};

    fn strnode<S: Into<String>>(s: S) -> SExpr { SExpr::String(s.into()) }
    fn idnode<S: Into<String>>(s: S) -> SExpr { SExpr::Ident(s.into()) }
    fn exact(i: i64) -> SExpr { SExpr::Number(Number::Exact(i)) }
    fn inexact(f: f64) -> SExpr { SExpr::Number(Number::Inexact(f)) }

    #[test]
    fn test_node_happy_path() {
        test_parse! {
            "'" => SExpr::Quote,
            "quote" => SExpr::Quote,
            "if" => SExpr::If,

            "#t" => SExpr::Bool(true),
            "#f" => SExpr::Bool(false),

            "12" => exact(12),
            "+12" => exact(12),
            "-12" => exact(-12),
            "0.5" => inexact(0.5),
            "+0.5" => inexact(0.5),
            "-0.5" => inexact(-0.5),

            "\"\"" => strnode(""),
            "\"abc\"" => strnode("abc"),
            r#""abc\"\\""# => strnode(r#"abc"\"#),

            "a1" => idnode("a1"),
            "id.ent+-" => idnode("id.ent+-"),
            "LoWeR" => idnode("lower"),
            "$" => idnode("$"),
            "+" => idnode("+"),
            "-" => idnode("-"),
            "..." => idnode("..."),

            "()" => SExpr::List(Vec::new()),
            "(#t 12 a)" => SExpr::List(vec![SExpr::Bool(true), exact(12), idnode("a")]),

            "(a . b)" => SExpr::DottedList(vec![idnode("a")], Box::new(idnode("b"))),
        };
    }

    #[test]
    fn test_invalid_node() {
        test_err! {
            "#",
            "#tr",
            "#fa",
            "#abc",

            ".1",
            "1.",

            "\"",
            "\"abc",
            r#""\""#,

            "+a",
            "-a",
            ".a",

            "(",
            "(#t 12 a",

            "(a .",
            "(. b",
        };
    }
}
