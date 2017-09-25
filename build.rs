use std::collections::HashMap;
use std::env;
use std::error::Error as StdError;
use std::fmt;
use std::iter::Peekable;
use std::fs::File;
use std::io::{Bytes, Read, Write, Error as IoError, Result as IoResult};
use std::path::Path;
use std::result::Result as StdResult;
use std::string::FromUtf8Error;

#[derive(Debug)]
enum Error {
    Custom(String),
    Wrap(Box<StdError>),
}
type Result<T> = StdResult<T, Error>;

macro_rules! custom_error {
    ( $( $vals: expr ),* ) => {
        return Err(Error::Custom(format!($( $vals ),*)))
    };
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Custom(ref s) => f.write_str(s),
            Error::Wrap(ref b) => b.fmt(f),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Custom(_) => "custom error",
            Error::Wrap(ref b) => b.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Custom(_) => None,
            Error::Wrap(ref b) => Some(b.as_ref()),
        }
    }
}

impl From<Box<StdError>> for Error {
    fn from(x: Box<StdError>) -> Error {
        Error::Wrap(x)
    }
}

macro_rules! impl_from_wrap {
    ( $( $ty:ty ),* ) => {
        $(
            impl From<$ty> for Error {
                fn from(x: $ty) -> Error {
                    Error::Wrap(Box::new(x))
                }
            }
        )*
    };
}

impl_from_wrap!(IoError, FromUtf8Error);

impl From<String> for Error {
    fn from(x: String) -> Error {
        Error::Custom(x)
    }
}

impl<'a> From<&'a str> for Error {
    fn from(x: &'a str) -> Error {
        Error::Custom(x.to_owned())
    }
}

struct Production {
    param: Option<String>,
    is_token: bool,
    inputs: Vec<ProductionInput>,
}

struct ProductionInput {
    output: Option<String>,
    format: Vec<FormatPart>,
}

struct FormatPart {
    attr: Option<MatcherAttribute>,
    pattern: FormatPattern,
}

enum MatcherAttribute {
    Many,
    Many1,
    Void,
}

enum FormatPattern {
    Production(String),
    Literal(String),
}

trait Pattern {
    fn consume_next(&self) -> bool;
    fn matches(&self, current: u8, next: Option<u8>) -> bool;
}

impl Pattern for u8 {
    fn consume_next(&self) -> bool {
        false
    }

    fn matches(&self, current: u8, _: Option<u8>) -> bool {
        &current == self
    }
}

impl<'a> Pattern for &'a [u8; 2] {
    fn consume_next(&self) -> bool {
        true
    }

    fn matches(&self, current: u8, next: Option<u8>) -> bool {
        current == self[0] && next == Some(self[1])
    }
}

type ByteIter = Peekable<Bytes<File>>;

fn read_until_opt<'a, 'b, I>(r: &mut Peekable<I>, pats: &[&'a Pattern]) -> Result<(Vec<u8>, Option<u8>)>
    where I: Iterator<Item=IoResult<u8>> {
    let mut out = Vec::new();
    let mut whitespace = Vec::new();
    let mut skipping = true;

    while let Some(rc) = r.next() {
        let c = rc?;

        if skipping && (c as char).is_whitespace() {
            continue;
        } else {
            skipping = false;
        }

        let (err, next) = match r.peek() {
            Some(&Ok(ref c)) => (false, Some(*c)),
            Some(&Err(_)) => (true, None),
            None => (false, None),
        };

        if err {
            r.next().unwrap()?;
        }

        for pat in pats {
            if pat.matches(c, next) {
                if pat.consume_next() {
                    r.next();
                }
                return Ok((out, Some(c)))
            }
        }
        if !(c as char).is_whitespace() {
            out.extend(whitespace.drain(..));
            out.push(c);
        } else {
            whitespace.push(c);
        }
    }
    Ok((out, None))
}

fn read_until<'a>(r: &mut ByteIter, pats: &[&'a Pattern]) -> Result<(Vec<u8>, u8)> {
    match read_until_opt(r, pats)? {
        (v, Some(c)) => Ok((v, c)),
        (_, None) => custom_error!("read_until"),
    }
}

struct PartialProduction {
    name: String,
    param: Option<String>,
    is_token: bool,
    remaining: Vec<u8>,
}

fn parse_left(source: &mut ByteIter) -> Result<PartialProduction> {
    let is_token = match source.peek() {
        Some(&Ok(b'_')) => false,
        _ => true,
    };

    if !is_token {
        source.next().unwrap()?;
    }

    let (ident, guard) = read_until(source, &[&b"->", &b'('])?;

    let param = if guard == b'(' {
        let (param, _) = read_until(source, &[&b')'])?;
        read_until(source, &[&b"->"])?;
        Some(String::from_utf8(param)?)
    } else {
        None
    };

    let (remaining, _) = read_until(source, &[&b'\n'])?;

    let partial =PartialProduction {
        name: String::from_utf8(ident)?,
        param: param,
        is_token: is_token,
        remaining: remaining,
    };

    Ok(partial)
}

fn parse_word(word: &[u8]) -> Result<FormatPart> {
    let (attr, char_fmt) = if word.len() > 1 {
        let (char_fmt, raw_attr) = word.split_at(word.len() - 1);
        match raw_attr[0] {
            b'*' => (Some(MatcherAttribute::Many), char_fmt),
            b'+' => (Some(MatcherAttribute::Many1), char_fmt),
            b'_' => (Some(MatcherAttribute::Void), char_fmt),
            _ => (None, word),
        }
    } else {
        (None, word)
    };

    let inner_pat = String::from_utf8(char_fmt.to_owned())?;
    let pattern = if char_fmt.len() > 1 && (char_fmt[0] as char).is_uppercase() {
        FormatPattern::Production(inner_pat)
    } else {
        FormatPattern::Literal(inner_pat)
    };

    Ok(FormatPart {
        attr: attr,
        pattern: pattern,
    })
}

fn parse_right(right: &[u8]) -> Result<Vec<ProductionInput>> {
    let mut source = right.iter().map(|c| Ok(*c)).peekable();
    let mut last_delim = Some(b'|');
    let mut inputs = Vec::new();
    while last_delim != None {
        let (raw_fmt, delim) = read_until_opt(&mut source, &[&b'|'])?;
        last_delim = delim;

        if raw_fmt.len() == 4 && raw_fmt[1] == b'.' && raw_fmt[2] == b'.' {
            for c in raw_fmt[0]..(raw_fmt[3] + 1) {
                let parts = vec![FormatPart {
                    attr: None,
                    pattern: FormatPattern::Literal(String::from_utf8(vec![c])?),
                }];
                inputs.push(ProductionInput {
                    output: None,
                    format: parts,
                });
            }
            continue;
        }

        let mut parts = Vec::new();
        let mut iter = raw_fmt.split(|c| (*c as char).is_whitespace());

        for word in &mut iter {
            if word == b"=>" {
                break;
            }
            parts.push(parse_word(word)?);
        }

        let output = if let Some(raw_output) = iter.next() {
            Some(String::from_utf8(raw_output.to_owned())?)
        } else {
            None
        };

        if iter.next() != None {
            custom_error!("Returns must be single words");
        }

        inputs.push(ProductionInput {
            output: output,
            format: parts,
        });
    }
    Ok(inputs)
}

fn parse_grammar(source: &mut ByteIter) -> Result<HashMap<String, Production>> {
    let mut partials = Vec::new();
    while source.peek().is_some() {
        partials.push(parse_left(source)?);
    }

    let mut result = HashMap::new();
    for partial in partials {
        result.insert(partial.name, Production {
            param: partial.param,
            is_token: partial.is_token,
            inputs: parse_right(&partial.remaining)?,
        });
    }

    Ok(result)
}

fn compile(input: File, output: &mut Write) -> Result<()> {
    let productions = parse_grammar(&mut input.bytes().peekable())?;
    for (name, prod) in productions {
        if !prod.is_token {
            output.write(b"_")?;
        }
        write!(output, "{}", name)?;
        if let Some(param) = prod.param {
            write!(output, "({})", param)?;
        }
        output.write(b" -> ")?;
        let mut is_first = true;
        for inp in &prod.inputs {
            if !is_first {
                output.write(b"| ")?;
            }
            is_first = false;

            for part in &inp.format {
                match part.pattern {
                    FormatPattern::Production(ref s) => { write!(output, "{}", s)?; }
                    FormatPattern::Literal(ref s) => { write!(output, "{}", s)?; }
                }
                if let Some(ref attr) = part.attr {
                    match *attr {
                        MatcherAttribute::Many => output.write(b"*")?,
                        MatcherAttribute::Many1 => output.write(b"+")?,
                        MatcherAttribute::Void => output.write(b"_")?,
                    };
                }

                if let Some(ref o) = inp.output {
                    write!(output, " => {}", o)?;
                }

                output.write(b" ")?;
            }
        }

        output.write(b"\n")?;
    }
    Ok(())
}

fn main() {
    let cwd = env::current_dir().unwrap();
    let source_path = Path::new(&cwd).join("grammar");
    let in_f = File::open(&source_path).unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("tokens.rs");
    let mut out_f = File::create(&dest_path).unwrap();

    compile(in_f, &mut out_f).unwrap();
}
