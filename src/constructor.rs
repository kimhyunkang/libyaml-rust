use document;
use document::YamlNodeData;
use ffi;
use parser::YamlMark;

use std::from_str::from_str;
use std::int;
use std::f64;
use std::u32;
use std::char;

pub trait YamlConstructor<T, E> {
    fn construct_scalar(&self, scalar: document::YamlScalarData) -> Result<T, E>;
    fn construct_sequence(&self, sequence: document::YamlSequenceData) -> Result<T, E>;
    fn construct_mapping(&self, mapping: document::YamlMappingData) -> Result<T, E>;

    fn construct<'r>(&self, node: document::YamlNode<'r>) -> Result<T, E> {
        match node {
            document::YamlScalarNode(scalar) => self.construct_scalar(scalar),
            document::YamlSequenceNode(sequence) => self.construct_sequence(sequence),
            document::YamlMappingNode(mapping) => self.construct_mapping(mapping)
        }
    }
}

#[deriving(PartialEq)]
#[deriving(Show)]
pub enum YamlStandardData {
    YamlInteger(int),
    YamlFloat(f64),
    YamlString(String),
    YamlNull,
    YamlBool(bool),
    YamlSequence(Vec<YamlStandardData>),
    YamlMapping(Vec<(YamlStandardData, YamlStandardData)>),
}

pub struct YamlStandardConstructor;

impl YamlStandardConstructor {
    pub fn new() -> YamlStandardConstructor {
        YamlStandardConstructor
    }

    pub fn parse_double_quoted(value: &str, mark: &YamlMark) -> Result<String, String> {
        let mut buf = String::new();
        let mut it = value.chars();

        loop {
            match it.next() {
                None => return Ok(buf),
                Some('\\') => {
                    // escape sequences
                    match it.next() {
                        None => return Err(format!("invalid escape sequence at line {:u}, col {:u}", mark.line, mark.column)),
                        Some('0') => buf.push_char('\x00'),
                        Some('a') => buf.push_char('\x07'),
                        Some('b') => buf.push_char('\x08'),
                        Some('t') | Some('\t') => buf.push_char('\t'),
                        Some('n') => buf.push_char('\n'),
                        Some('v') => buf.push_char('\x0b'),
                        Some('f') => buf.push_char('\x0c'),
                        Some('r') => buf.push_char('\x0d'),
                        Some('e') => buf.push_char('\x1b'),
                        Some('N') => buf.push_char('\x85'),
                        Some('_') => buf.push_char('\xa0'),
                        Some('L') => buf.push_char('\u2028'),
                        Some('P') => buf.push_char('\u2029'),
                        Some('x') => {
                            let code:String = it.take(2).collect();
                            match parse_escape_sequence(code, 2) {
                                Some(c) => buf.push_char(c),
                                None => return Err(format!("invalid x escape sequence at line {:u}, col {:u}", mark.line, mark.column))
                            }
                        },
                        Some('u') => {
                            let code:String = it.take(4).collect();
                            match parse_escape_sequence(code, 4) {
                                Some(c) => buf.push_char(c),
                                None => return Err(format!("invalid x escape sequence at line {:u}, col {:u}", mark.line, mark.column))
                            }
                        },
                        Some('U') => {
                            let code:String = it.take(8).collect();
                            match parse_escape_sequence(code, 8) {
                                Some(c) => buf.push_char(c),
                                None => return Err(format!("invalid x escape sequence at line {:u}, col {:u}", mark.line, mark.column))
                            }
                        },
                        Some(c) => buf.push_char(c)
                    }
                },
                Some(c) => buf.push_char(c)
            }
        }
    }
}

fn parse_escape_sequence(rep: String, expected_len: uint) -> Option<char> {
    match u32::parse_bytes(rep.as_bytes(), 16) {
        Some(code) if rep.len() == expected_len => char::from_u32(code),
        _ => None
    }
}

fn parse_int(sign: &str, data: &str, radix: uint) -> int {
    let sign_flag = if sign == "-" {
            -1
        } else {
            1
        };

    let filtered:String = data.chars().filter(|&c| c != '_').collect();
    int::parse_bytes(filtered.as_bytes(), radix).unwrap() * sign_flag
}

impl YamlConstructor<YamlStandardData, String> for YamlStandardConstructor {
    fn construct_scalar(&self, scalar: document::YamlScalarData) -> Result<YamlStandardData, String> {
        let dec_int = regex!(r"^[-+]?(0|[1-9][0-9_]*)$");
        let oct_int = regex!(r"^([-+]?)0o?([0-7_]+)$");
        let hex_int = regex!(r"^([-+]?)0x([0-9a-fA-F_]+)$");
        let bin_int = regex!(r"^([-+]?)0b([0-1_]+)$");
        let float_pattern = regex!(r"^[-+]?(\.[0-9]+|[0-9]+(\.[0-9]*)?)([eE][-+]?[0-9]+)?$");
        let pos_inf = regex!(r"^[+]?(\.inf|\.Inf|\.INF)$");
        let neg_inf = regex!(r"^-(\.inf|\.Inf|\.INF)$");
        let nan = regex!(r"^(\.nan|\.NaN|\.NAN)$");
        let null = regex!(r"^(null|Null|NULL|~)$");
        let true_pattern = regex!(r"^(true|True|TRUE|yes|Yes|YES)$");
        let false_pattern = regex!(r"^(false|False|FALSE|no|No|NO)$");

        let value = scalar.get_value();
        let mark = scalar.start_mark();

        match scalar.style() {
            ffi::YamlPlainScalarStyle => {
                match bin_int.captures(value.as_slice()) {
                    Some(caps) => return Ok(YamlInteger(parse_int(caps.at(1), caps.at(2), 2))),
                    None => ()
                };
                match oct_int.captures(value.as_slice()) {
                    Some(caps) => return Ok(YamlInteger(parse_int(caps.at(1), caps.at(2), 8))),
                    None => ()
                };
                match hex_int.captures(value.as_slice()) {
                    Some(caps) => return Ok(YamlInteger(parse_int(caps.at(1), caps.at(2), 16))),
                    None => ()
                };

                if dec_int.is_match(value.as_slice()) {
                    Ok(YamlInteger(parse_int("", value.as_slice(), 10)))
                } else if float_pattern.is_match(value.as_slice()) {
                    Ok(YamlFloat(from_str(value.as_slice()).unwrap()))
                } else if pos_inf.is_match(value.as_slice()) {
                    Ok(YamlFloat(f64::INFINITY))
                } else if neg_inf.is_match(value.as_slice()) {
                    Ok(YamlFloat(f64::NEG_INFINITY))
                } else if nan.is_match(value.as_slice()) {
                    Ok(YamlFloat(f64::NAN))
                } else if null.is_match(value.as_slice()) {
                    Ok(YamlNull)
                } else if true_pattern.is_match(value.as_slice()) {
                    Ok(YamlBool(true))
                } else if false_pattern.is_match(value.as_slice()) {
                    Ok(YamlBool(false))
                } else {
                    Ok(YamlString(value))
                }
            },
            ffi::YamlDoubleQuotedScalarStyle => {
                YamlStandardConstructor::parse_double_quoted(value.as_slice(), &mark).map(YamlString)
            },
            _ => {
                Ok(YamlString(value))
            }
        }
    }

    fn construct_sequence(&self, sequence: document::YamlSequenceData) -> Result<YamlStandardData, String> {
        let res:Result<Vec<YamlStandardData>, String> = sequence.values().map(|node| { self.construct(node) }).collect();
        res.map(|list| YamlSequence(list))
    }

    fn construct_mapping(&self, mapping: document::YamlMappingData) -> Result<YamlStandardData, String> {
        let mut pairs = mapping.pairs().map(|(key_node, value_node)| {
            match self.construct(key_node) {
                Ok(key) => match self.construct(value_node) {
                    Ok(value) => Ok((key, value)),
                    Err(e) => Err(e)
                },
                Err(e) => Err(e)
            }
        });
        let res:Result<Vec<(YamlStandardData, YamlStandardData)>, String> = pairs.collect();
        res.map(YamlMapping)
    }
}

#[cfg(test)]
mod test {
    use parser::{YamlParser, YamlByteParser};
    use std::f64;
    use ffi::YamlUtf8Encoding;
    use constructor::{YamlConstructor, YamlStandardConstructor, YamlInteger, YamlFloat, YamlBool, YamlNull, YamlString, YamlSequence};

    #[test]
    fn test_standard_constructor() {
        let data = "[1, 2, 3]";
        let parser = YamlByteParser::init(data.as_bytes(), YamlUtf8Encoding);

        match parser.load().next() {
            Some(Ok(doc)) => {
                let ctor = YamlStandardConstructor::new();
                assert_eq!(Ok(YamlSequence(vec![YamlInteger(1), YamlInteger(2), YamlInteger(3)])), ctor.construct(doc.root().unwrap()))
            },
            err => fail!("unexpected result: {:?}", err)
        }
    }

    #[test]
    fn test_integer_parser() {
        let data = "[0o10, 0x21, -30]";
        let parser = YamlByteParser::init(data.as_bytes(), YamlUtf8Encoding);

        match parser.load().next() {
            Some(Ok(doc)) => {
                let ctor = YamlStandardConstructor::new();
                assert_eq!(Ok(YamlSequence(vec![YamlInteger(0o10), YamlInteger(0x21), YamlInteger(-30)])), ctor.construct(doc.root().unwrap()))
            },
            err => fail!("unexpected result: {:?}", err)
        }
    }

    #[test]
    fn test_float_parser() {
        let data = "[0.3, -.4, 1e+2, -1.2e-3]";
        let parser = YamlByteParser::init(data.as_bytes(), YamlUtf8Encoding);

        match parser.load().next() {
            Some(Ok(doc)) => {
                let ctor = YamlStandardConstructor::new();
                let value = ctor.construct(doc.root().unwrap());
                match value {
                    Ok(YamlSequence(seq)) => {
                        match seq.as_slice() {
                            [YamlFloat(f1), YamlFloat(f2), YamlFloat(f3), YamlFloat(f4)] => {
                                assert!((f1 - 0.3).abs() < 1.0e-6);
                                assert!((f2 + 0.4).abs() < 1.0e-6);
                                assert!((f3 - 1e+2).abs() < 1.0e-6);
                                assert!((f4 + 1.2e-3).abs() < 1.0e-6);
                            },
                            _ => fail!("unexpected sequence: {:?}", seq)
                        }
                    },
                    _ => fail!("unexpected result: {:?}", value)
                }
            },
            err => fail!("document parse failure: {:?}", err)
        }
    }

    #[test]
    fn test_inf_parser() {
        let data = "[.inf, -.INF]";
        let parser = YamlByteParser::init(data.as_bytes(), YamlUtf8Encoding);

        match parser.load().next() {
            Some(Ok(doc)) => {
                let ctor = YamlStandardConstructor::new();
                assert_eq!(Ok(YamlSequence(vec![YamlFloat(f64::INFINITY), YamlFloat(f64::NEG_INFINITY)])), ctor.construct(doc.root().unwrap()))
            },
            err => fail!("document parse failure: {:?}", err)
        }
    }

    #[test]
    fn test_misc_parser() {
        let data = "[yes, False, ~]";
        let parser = YamlByteParser::init(data.as_bytes(), YamlUtf8Encoding);

        match parser.load().next() {
            Some(Ok(doc)) => {
                let ctor = YamlStandardConstructor::new();
                assert_eq!(Ok(YamlSequence(vec![YamlBool(true), YamlBool(false), YamlNull])), ctor.construct(doc.root().unwrap()))
            },
            err => fail!("document parse failure: {:?}", err)
        }
    }

    #[test]
    fn test_double_quoted_parser() {
        let data = r#""hello, \"world\"""#;
        let parser = YamlByteParser::init(data.as_bytes(), YamlUtf8Encoding);

        match parser.load().next() {
            Some(Ok(doc)) => {
                let ctor = YamlStandardConstructor::new();
                assert_eq!(Ok(YamlString("hello, \"world\"".to_string())), ctor.construct(doc.root().unwrap()))
            },
            err => fail!("document parse failure: {:?}", err)
        }
    }

    #[test]
    fn test_single_quoted_parser() {
        let data = r#"'here''s to "quotes"'"#;
        let parser = YamlByteParser::init(data.as_bytes(), YamlUtf8Encoding);

        match parser.load().next() {
            Some(Ok(doc)) => {
                let ctor = YamlStandardConstructor::new();
                assert_eq!(Ok(YamlString(r#"here's to "quotes""#.to_string())), ctor.construct(doc.root().unwrap()))
            },
            err => fail!("document parse failure: {:?}", err)
        }
    }

    #[test]
    fn test_underlined_integer() {
        let data = "[1_000, -2_000_000]";
        let parser = YamlByteParser::init(data.as_bytes(), YamlUtf8Encoding);

        match parser.load().next() {
            Some(Ok(doc)) => {
                let ctor = YamlStandardConstructor::new();
                assert_eq!(Ok(YamlSequence(vec![YamlInteger(1000), YamlInteger(-2000000)])), ctor.construct(doc.root().unwrap()))
            },
            err => fail!("document parse failure: {:?}", err)
        }
    }

    #[test]
    fn test_negative_radix() {
        let data = "[-0x30, -0700, -0b110]";
        let parser = YamlByteParser::init(data.as_bytes(), YamlUtf8Encoding);

        match parser.load().next() {
            Some(Ok(doc)) => {
                let ctor = YamlStandardConstructor::new();
                assert_eq!(Ok(YamlSequence(vec![YamlInteger(-48), YamlInteger(-448), YamlInteger(-6)])), ctor.construct(doc.root().unwrap()))
            },
            err => fail!("document parse failure: {:?}", err)
        }
    }
}
