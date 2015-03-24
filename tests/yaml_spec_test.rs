// Allow unstable items until Rust hits 1.0

extern crate yaml;

use yaml::constructor::YamlStandardData;
use yaml::ffi::YamlEncoding;
use std::io::Cursor;

macro_rules! test_utf8{
    ($filename: expr, $expected: expr) => (
        test_file!(yaml::ffi::YamlEncoding::YamlUtf8Encoding, $filename, $expected)
    )
}

macro_rules! test_file{
    ($encoding: expr, $filename: expr, $expected: expr) => (
        {
            let data: &[u8] = include_bytes!($filename);
            let mut reader = Cursor::new(data);
            match yaml::parse_io(&mut reader, $encoding) {
                Ok(docs) => if docs.len() == 1 {
                    assert_eq!(docs[..].first().unwrap(), &$expected)
                } else {
                    panic!("too many number of documents: {:?}", docs)
                },
                Err(e) => panic!("parse failure: {:?}", e)
            }
        }
    )
}

macro_rules! ystr{
    ($e: expr) => (
        YamlStandardData::YamlString($e.to_string())
    )
}

macro_rules! yint{
    ($e: expr) => (
        YamlStandardData::YamlInteger($e)
    )
}

macro_rules! yfloat{
    ($e: expr) => (
        YamlStandardData::YamlFloat($e)
    )
}

macro_rules! yseq{
    ($($e:expr),*) => (
        YamlStandardData::YamlSequence(vec![$(($e),)*])
    )
}

macro_rules! ymap{
    ($($k:expr => $v:expr),*) => (
        YamlStandardData::YamlMapping(vec![$((ystr!($k), $v),)*])
    )
}

macro_rules! y_cmp_map{
    ($($k:expr => $v:expr),*) => (
        YamlStandardData::YamlMapping(vec![$(($k, $v),)*])
    )
}

#[test]
fn sequence_of_scalars() {
    test_utf8!("source/ball_players.yml", yseq![ystr!("Mark McGwire"), ystr!("Sammy Sosa"), ystr!("Ken Griffey")]);
}

#[test]
fn scalar_mappings() {
    test_utf8!("source/player_stat.yml", ymap!{
                                    "hr" => yint!(65),
                                    "avg" => yfloat!(0.278),
                                    "rbi" => yint!(147)
                                })
}

#[test]
fn maps_of_sequences() {
    test_utf8!("source/ball_clubs.yml", ymap!{
                                    "american" => yseq![ystr!("Boston Red Sox"), ystr!("Detroit Tigers"), ystr!("New York Yankees")],
                                    "national" => yseq![ystr!("New York Mets"), ystr!("Chicago Cubs"), ystr!("Atlanta Braves")]
                                })
}

#[test]
fn sequence_of_maps() {
    test_utf8!("source/multiple_player_stat.yml",
    yseq![
        ymap!{
            "name" => ystr!("Mark McGwire"),
            "hr" => yint!(65)
        },
        ymap!{
            "name" => ystr!("Sammy Sosa"),
            "hr" => yint!(63)
        }
    ])
}

#[test]
fn sequence_of_sequences() {
    test_utf8!("source/csv.yml",
    yseq![
        yseq![ystr!("name"), ystr!("hr")],
        yseq![ystr!("Mark McGwire"), yint!(65)],
        yseq![ystr!("Sammy Sosa"), yint!(63)]
    ])
}

#[test]
fn mapping_of_mappings() {
    test_utf8!("source/map_map.yml",
    ymap!{
        "Mark McGwire" => ymap!{ "hr" => yint!(65) },
        "Sammy Sosa" => ymap!{ "hr" => yint!(63) }
    })
}

#[test]
fn alias() {
    test_utf8!("source/alias.yml",
    ymap!{
        "hr" => yseq![ystr!("Mark McGwire"), ystr!("Sammy Sosa")],
        "rbi" => yseq![ystr!("Sammy Sosa"), ystr!("Ken Griffey")]
    })
}

#[test]
fn complex_keys() {
    test_utf8!("source/complex_key.yml",
    y_cmp_map!{
        yseq![ystr!("Detroit Tigers"), ystr!("Chicago Cubs")] => yseq![ystr!("2001-07-23")],
        yseq![ystr!("New York Yankees"), ystr!("Atlanta Braves")] => yseq![ystr!("2001-07-02"), ystr!("2001-08-12"), ystr!("2001-08-14")]
    })
}

#[test]
fn block_literal() {
    test_utf8!("source/block_literal.yml", ystr!("\\//||\\/||\n// ||  ||__\n"))
}

#[test]
fn plain_scalar() {
    test_utf8!("source/plain_scalar.yml", ystr!("Mark McGwire's year was crippled by a knee injury."))
}

#[test]
fn quoted_scalar() {
    test_utf8!("source/quoted_scalar.yml",
    ymap!{
        "unicode" => ystr!("Sosa did fine.\u{263A}"),
        "control" => ystr!("\x081998\t1999\t2000\n"),
        "hexesc" =>  ystr!("\x13\x10 is \r\n"),
        "single" => ystr!(r#""Howdy!" he cried."#),
        "quoted" => ystr!(" # not a 'comment'."),
        "tie-fighter" => ystr!(r"|\-*-/|")
    })
}

#[test]
fn multi_line_scalar() {
    test_utf8!("source/multi_line_scalar.yml",
    ymap!{
        "plain" => ystr!("This unquoted scalar spans many lines."),
        "quoted" => ystr!("So does this quoted scalar.\n")
    })
}

#[test]
fn utf16le() {
    test_file!(YamlEncoding::YamlUtf16LeEncoding, "source/utf16le.yml",
               yseq![ystr!("Hello"), ystr!("世界")]
    )
}

#[test]
fn utf16be() {
    test_file!(YamlEncoding::YamlUtf16BeEncoding, "source/utf16be.yml",
               yseq![ystr!("Hello"), ystr!("世界")]
    )
}
