#![feature(macro_rules)]

extern crate debug;
extern crate yaml;

use yaml::constructor::{YamlStandardData, YamlMapping, YamlSequence, YamlString, YamlInteger, YamlFloat};
use std::os;
use std::io::{File, BufferedReader};

fn match_file(filename: &str, expected: YamlStandardData) {
    let this_path = os::args().as_slice()[0].clone();
    let file_path = Path::new(this_path).join("../../test/source").join(filename);
    println!("{}", file_path.display())
    let mut reader = BufferedReader::new(File::open(&file_path));
    match yaml::parse_io(&mut reader) {
        Ok(docs) => if docs.len() == 1 {
            assert_eq!(docs.as_slice().head().unwrap(), &expected)
        } else {
            fail!("too many number of documents: {:?}", docs)
        },
        Err(e) => fail!("parse failure: {:?}", e)
    }
}

macro_rules! ystr(
    ($e: expr) => (
        YamlString($e.to_string())
    )
)

macro_rules! yint(
    ($e: expr) => (
        YamlInteger($e)
    )
)

macro_rules! yfloat(
    ($e: expr) => (
        YamlFloat($e)
    )
)

macro_rules! yseq(
    ($($e:expr),*) => (
        YamlSequence(vec![$(($e),)*])
    )
)

macro_rules! ymap(
    ($($k:expr : $v:expr),*) => (
        YamlMapping(vec![$((ystr!($k), $v),)*])
    )
)

macro_rules! y_cmp_map(
    ($($k:expr : $v:expr),*) => (
        YamlMapping(vec![$(($k, $v),)*])
    )
)

#[test]
fn sequence_of_scalars() {
    match_file("ball_players.yml", yseq![ystr!("Mark McGwire"), ystr!("Sammy Sosa"), ystr!("Ken Griffey")]);
}

#[test]
fn scalar_mappings() {
    match_file("player_stat.yml", ymap!{
                                    "hr": yint!(65),
                                    "avg": yfloat!(0.278),
                                    "rbi": yint!(147)
                                })
}

#[test]
fn maps_of_sequences() {
    match_file("ball_clubs.yml", ymap!{
                                    "american": yseq![ystr!("Boston Red Sox"), ystr!("Detroit Tigers"), ystr!("New York Yankees")],
                                    "national": yseq![ystr!("New York Mets"), ystr!("Chicago Cubs"), ystr!("Atlanta Braves")]
                                })
}

#[test]
fn sequence_of_maps() {
    match_file("multiple_player_stat.yml",
    yseq![
        ymap!{
            "name": ystr!("Mark McGwire"),
            "hr": yint!(65)
        },
        ymap!{
            "name": ystr!("Sammy Sosa"),
            "hr": yint!(63)
        }
    ])
}

#[test]
fn sequence_of_sequences() {
    match_file("csv.yml",
    yseq![
        yseq![ystr!("name"), ystr!("hr")],
        yseq![ystr!("Mark McGwire"), yint!(65)],
        yseq![ystr!("Sammy Sosa"), yint!(63)]
    ])
}

#[test]
fn mapping_of_mappings() {
    match_file("map_map.yml",
    ymap!{
        "Mark McGwire": ymap!{ "hr": yint!(65) },
        "Sammy Sosa": ymap!{ "hr": yint!(63) }
    })
}

#[test]
fn alias() {
    match_file("alias.yml",
    ymap!{
        "hr": yseq![ystr!("Mark McGwire"), ystr!("Sammy Sosa")],
        "rbi": yseq![ystr!("Sammy Sosa"), ystr!("Ken Griffey")]
    })
}

#[test]
fn complex_keys() {
    match_file("complex_key.yml",
    y_cmp_map!{
        yseq![ystr!("Detroit Tigers"), ystr!("Chicago Cubs")]: yseq![ystr!("2001-07-23")],
        yseq![ystr!("New York Yankees"), ystr!("Atlanta Braves")]: yseq![ystr!("2001-07-02"), ystr!("2001-08-12"), ystr!("2001-08-14")]
    })
}

#[test]
fn block_literal() {
    match_file("block_literal.yml", ystr!("\\//||\\/||\n// ||  ||__\n"))
}

#[test]
fn plain_scalar() {
    match_file("plain_scalar.yml", ystr!("Mark McGwire's year was crippled by a knee injury."))
}

#[test]
fn quoted_scalar() {
    match_file("quoted_scalar.yml",
    ymap!{
        "unicode": ystr!("Sosa did fine.\u263A"),
        "control": ystr!("\x081998\t1999\t2000\n"),
        "hexesc":  ystr!("\x13\x10 is \r\n"),
        "single": ystr!(r#""Howdy!" he cried."#),
        "quoted": ystr!(" # not a 'comment'."),
        "tie-fighter": ystr!(r"|\-*-/|")
    })
}

#[test]
fn multi_line_scalar() {
    match_file("multi_line_scalar.yml",
    ymap!{
        "plain": ystr!("This unquoted scalar spans many lines."),
        "quoted": ystr!("So does this quoted scalar.\n")
    })
}
