use document;
use document::{YamlNode, YamlNodeData};
use ffi::{YamlErrorType, YamlScalarStyle};
use error::{YamlMark, YamlError, YamlErrorContext};

use std::f64;
use std::char;
use regex::Regex;

pub trait YamlConstructor<T, E> {
    fn construct_scalar(&self, scalar: document::YamlScalarData) -> Result<T, E>;
    fn construct_sequence(&self, sequence: document::YamlSequenceData) -> Result<T, E>;
    fn construct_mapping(&self, mapping: document::YamlMappingData) -> Result<T, E>;

    fn construct<'r>(&self, node: document::YamlNode<'r>) -> Result<T, E> {
        match node {
            YamlNode::YamlScalarNode(scalar) => self.construct_scalar(scalar),
            YamlNode::YamlSequenceNode(sequence) => self.construct_sequence(sequence),
            YamlNode::YamlMappingNode(mapping) => self.construct_mapping(mapping)
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum YamlStandardData {
    YamlInteger(isize),
    YamlFloat(f64),
    YamlString(String),
    YamlNull,
    YamlBool(bool),
    YamlSequence(Vec<YamlStandardData>),
    YamlMapping(Vec<(YamlStandardData, YamlStandardData)>),
}

#[derive(Clone)]
pub struct YamlStandardConstructor {
    dec_int_pat:Regex,
    oct_int_pat:Regex,
    hex_int_pat:Regex,
    bin_int_pat:Regex,
    flt_pat:Regex,
    pos_inf_pat:Regex,
    neg_inf_pat:Regex,
    nan_pat:Regex,
    null_pat:Regex,
    true_pat:Regex,
    false_pat:Regex
}

fn standard_error(message: String, mark: &YamlMark) -> YamlError {
    let context = YamlErrorContext {
        byte_offset: mark.index,
        problem_mark: *mark,
        context: None,
        context_mark: *mark,
    };

    YamlError {
        kind: YamlErrorType::YAML_PARSER_ERROR,
        problem: Some(message),
        io_error: None,
        context: Some(context)
    }
}

fn take(iter: &mut Iterator<Item=char>, n: usize) -> String
{
    let mut s = String::new();
    for _ in 0..n {
        match iter.next() {
            Some(c) => s.push(c),
            None => break
        }
    }
    return s;
}

impl YamlStandardConstructor {
    pub fn new() -> YamlStandardConstructor {
        YamlStandardConstructor {
            dec_int_pat: Regex::new(r"^[-+]?(0|[1-9][0-9_]*)$").unwrap(),
            oct_int_pat: Regex::new(r"^([-+]?)0o?([0-7_]+)$").unwrap(),
            hex_int_pat: Regex::new(r"^([-+]?)0x([0-9a-fA-F_]+)$").unwrap(),
            bin_int_pat: Regex::new(r"^([-+]?)0b([0-1_]+)$").unwrap(),
            flt_pat: Regex::new(r"^([-+]?)(\.[0-9]+|[0-9]+(\.[0-9]*)?([eE][-+]?[0-9]+)?)$").unwrap(),
            pos_inf_pat: Regex::new(r"^[+]?(\.inf|\.Inf|\.INF)$").unwrap(),
            neg_inf_pat: Regex::new(r"^-(\.inf|\.Inf|\.INF)$").unwrap(),
            nan_pat: Regex::new(r"^(\.nan|\.NaN|\.NAN)$").unwrap(),
            null_pat: Regex::new(r"^(null|Null|NULL|~)$").unwrap(),
            true_pat: Regex::new(r"^(true|True|TRUE|yes|Yes|YES)$").unwrap(),
            false_pat: Regex::new(r"^(false|False|FALSE|no|No|NO)$").unwrap()
        }
    }

    fn parse_double_quoted(value: &str, mark: &YamlMark) -> Result<String, YamlError> {
        let mut buf = String::new();
        let mut it = value.chars();

        loop {
            match it.next() {
                None => return Ok(buf),
                Some('\\') => {
                    // escape sequences
                    match it.next() {
                        None => return Err(standard_error(
                                    "unexpected end of string after escape".to_string(),
                                    mark
                                )),
                        Some('0') => buf.push('\x00'),              // null
                        Some('a') => buf.push('\x07'),              // ASCII bell
                        Some('b') => buf.push('\x08'),              // backspace
                        Some('t') | Some('\t') => buf.push('\t'),   // horizontal tab
                        Some('n') => buf.push('\n'),                // linefeed
                        Some('v') => buf.push('\x0b'),              // vertical tab
                        Some('f') => buf.push('\x0c'),              // form feed
                        Some('r') => buf.push('\x0d'),              // carriage return
                        Some('e') => buf.push('\x1b'),              // ASCII escape
                        Some('N') => buf.push('\u{85}'),            // unicode next line
                        Some('_') => buf.push('\u{a0}'),            // unicode non-breaking space
                        Some('L') => buf.push('\u{2028}'),          // unicode line separator
                        Some('P') => buf.push('\u{2029}'),          // unicode paragraph separator
                        Some('x') => {
                            let code:String = take(&mut it, 2);
                            match parse_escape_sequence(&code[..], 2) {
                                Some(c) => buf.push(c),
                                None => return Err(standard_error(
                                            format!("invalid escape sequence {}", code),
                                            mark
                                        ))
                            }
                        },
                        Some('u') => {
                            let code:String = take(&mut it, 4);
                            match parse_escape_sequence(&code[..], 4) {
                                Some(c) => buf.push(c),
                                None => return Err(standard_error(
                                            format!("invalid escape sequence {}", code),
                                            mark
                                        ))
                            }
                        },
                        Some('U') => {
                            let code:String = take(&mut it, 8);
                            match parse_escape_sequence(&code[..], 8) {
                                Some(c) => buf.push(c),
                                None => return Err(standard_error(
                                            format!("invalid escape sequence {}", code),
                                            mark
                                        ))
                            }
                        },
                        Some(c) => buf.push(c)
                    }
                },
                Some(c) => buf.push(c)
            }
        }
    }
}

fn parse_escape_sequence(rep: &str, expected_len: usize) -> Option<char> {
    match u32::from_str_radix(rep, 16) {
        Ok(code) if rep.len() == expected_len => char::from_u32(code),
        _ => None
    }
}

fn parse_int(sign: &str, data: &str, radix: u32) -> isize {
    let sign_flag = if sign == "-" {
            -1
        } else {
            1
        };

    let filtered:String = data.chars().filter(|&c| c != '_').collect();
    let unsigned:isize = isize::from_str_radix(&filtered[..], radix).unwrap();
    return unsigned * sign_flag;
}

fn parse_float(sign: &str, data: &str) -> f64 {
    let unsigned:f64 = data.parse().unwrap();
    if sign == "-" {
        return -unsigned;
    } else {
        return unsigned;
    }
}

impl YamlConstructor<YamlStandardData, YamlError> for YamlStandardConstructor {
    fn construct_scalar(&self, scalar: document::YamlScalarData) -> Result<YamlStandardData, YamlError> {
        let value = scalar.get_value();
        let mark = scalar.start_mark();

        match scalar.style() {
            YamlScalarStyle::YamlPlainScalarStyle => {
                match self.bin_int_pat.captures(&value[..]) {
                    Some(caps) => return Ok(YamlStandardData::YamlInteger(parse_int(
                                &caps[1], &caps[2], 2))),
                    None => ()
                };
                match self.oct_int_pat.captures(&value[..]) {
                    Some(caps) => return Ok(YamlStandardData::YamlInteger(parse_int(
                                &caps[1], &caps[2], 8))),
                    None => ()
                };
                match self.hex_int_pat.captures(&value[..]) {
                    Some(caps) => return Ok(YamlStandardData::YamlInteger(parse_int(
                                &caps[1], &caps[2], 16))),
                    None => ()
                };

                if self.dec_int_pat.is_match(&value[..]) {
                    return Ok(YamlStandardData::YamlInteger(parse_int("", &value[..], 10)));
                }

                match self.flt_pat.captures(&value[..]) {
                    Some(caps) => return Ok(YamlStandardData::YamlFloat(parse_float(
                                &caps[1], &caps[2]))),
                    None => ()
                };

                if self.pos_inf_pat.is_match(&value[..]) {
                    Ok(YamlStandardData::YamlFloat(f64::INFINITY))
                } else if self.neg_inf_pat.is_match(&value[..]) {
                    Ok(YamlStandardData::YamlFloat(f64::NEG_INFINITY))
                } else if self.nan_pat.is_match(&value[..]) {
                    Ok(YamlStandardData::YamlFloat(f64::NAN))
                } else if self.null_pat.is_match(&value[..]) {
                    Ok(YamlStandardData::YamlNull)
                } else if self.true_pat.is_match(&value[..]) {
                    Ok(YamlStandardData::YamlBool(true))
                } else if self.false_pat.is_match(&value[..]) {
                    Ok(YamlStandardData::YamlBool(false))
                } else {
                    Ok(YamlStandardData::YamlString(value))
                }
            },
            YamlScalarStyle::YamlDoubleQuotedScalarStyle => {
                YamlStandardConstructor::parse_double_quoted(&value[..], &mark).map(YamlStandardData::YamlString)
            },
            _ => {
                Ok(YamlStandardData::YamlString(value))
            }
        }
    }

    fn construct_sequence(&self, sequence: document::YamlSequenceData) -> Result<YamlStandardData, YamlError> {
        let res:Result<Vec<YamlStandardData>, YamlError> = sequence.values().map(|node| { self.construct(node) }).collect();
        res.map(|list| YamlStandardData::YamlSequence(list))
    }

    fn construct_mapping(&self, mapping: document::YamlMappingData) -> Result<YamlStandardData, YamlError> {
        let pairs = mapping.pairs().map(|(key_node, value_node)| {
            match self.construct(key_node) {
                Ok(key) => match self.construct(value_node) {
                    Ok(value) => Ok((key, value)),
                    Err(e) => Err(e)
                },
                Err(e) => Err(e)
            }
        });
        let res:Result<Vec<(YamlStandardData, YamlStandardData)>, YamlError> = pairs.collect();
        res.map(YamlStandardData::YamlMapping)
    }
}

#[cfg(test)]
mod test {
    use super::YamlStandardData::*;
    use parser::{YamlParser, YamlByteParser};
    use std::f64;
    use ffi::YamlEncoding::YamlUtf8Encoding;
    use constructor::{YamlConstructor, YamlStandardConstructor};

    #[test]
    fn test_standard_constructor() {
        let data = "[1, 2, 3]";
        let parser = YamlByteParser::init(data.as_bytes(), YamlUtf8Encoding);

        match parser.load().next() {
            Some(Ok(doc)) => {
                let ctor = YamlStandardConstructor::new();
                assert_eq!(Ok(YamlSequence(vec![YamlInteger(1), YamlInteger(2), YamlInteger(3)])), ctor.construct(doc.root().unwrap()))
            },
            _ => panic!("unexpected result")
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
            _ => panic!("unexpected result")
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
                        match &seq[..] {
                            &[YamlFloat(f1), YamlFloat(f2), YamlFloat(f3), YamlFloat(f4)] => {
                                assert!((f1 - 0.3).abs() < 1.0e-6);
                                assert!((f2 + 0.4) < 1.0e-6);
                                assert!((f3 - 1e+2) < 1.0e-6);
                                assert!((f4 + 1.2e-3) < 1.0e-6);
                            },
                            _ => panic!("unexpected sequence")
                        }
                    },
                    _ => panic!("unexpected result")
                }
            },
            _ => panic!("document parse failure")
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
            _ => panic!("document parse failure")
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
            _ => panic!("document parse failure")
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
            _ => panic!("document parse failure")
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
            _ => panic!("document parse failure")
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
            _ => panic!("document parse failure")
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
            _ => panic!("document parse failure")
        }
    }
}
