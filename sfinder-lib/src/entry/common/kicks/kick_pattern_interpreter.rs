use std::cell::OnceCell;

use crate::{
    common::datastore::coordinate::Coordinate, entry::common::kicks::kick_pattern::KickPatternType,
    sfinder_core::srs::pattern::_Pattern,
};

use super::{kick_pattern::KickPattern, kick_type::KickType};
use regex_lite::Regex;

struct XYMark {
    pub x: i8,
    pub y: i8,
    pub mark: bool,
}

pub fn create(key: String, value: String) -> Result<KickPattern, String> {
    let trimmed_key = key.trim();
    let trimmed_value = value.replace(" ", "");

    let kick_type = parse_to_kick_type(trimmed_key)?;
    let kick_pattern_type = if let Some(stripped_value) = trimmed_value.strip_prefix('&') {
        KickPatternType::Referenced {
            reference_kick_type: parse_to_kick_type(stripped_value)?,
        }
    } else {
        KickPatternType::Fixed {
            pattern: parse_to_pattern(trimmed_value)?,
        }
    };

    KickPattern::try_new(kick_type, kick_pattern_type)
}

fn parse_to_kick_type(str: &str) -> Result<KickType, String> {
    const RE: OnceCell<Regex> = OnceCell::new();
    let capture = RE
        .get_or_init(|| Regex::new(r"[TIOLJSZ]\.[NEWSLR02]{2}").unwrap())
        .find(&str)
        .ok_or_else(|| format!("Cannot parse kick type: value={str}"))?
        .as_str();

    let from = capture[2..3].parse().unwrap();
    let to = capture[3..4].parse().unwrap();

    if from == to {
        return Err(format!("Kick type is invalid: value={str}"));
    }

    Ok(KickType {
        piece: capture[0..1].parse().unwrap(),
        from,
        to,
    })
}

fn parse_to_pattern(str: String) -> Result<_Pattern, String> {
    validate(&str)?;

    let brackets = detect_brackets(&str)?;
    let brackets_len = brackets.len();

    if brackets.is_empty() {
        return Err(format!("Invalid value: value={str}"));
    }

    let xy_marks = detect_xys(brackets)?;

    assert_eq!(brackets_len, xy_marks.len());

    Ok(create_pattern(xy_marks))
}

fn validate(str: &str) -> Result<(), String> {
    if str.is_empty() {
        return Err("empty string".to_string());
    }

    let open = str.chars().filter(|&c| c == '(').count();
    let close = str.chars().filter(|&c| c == ')').count();

    if open != close {
        return Err("brackets are not balanced".to_string());
    }
    if open == 0 {
        return Err("no brackets found".to_string());
    }

    Ok(())
}

fn detect_brackets(str: &str) -> Result<Vec<&str>, String> {
    const RE: OnceCell<Regex> = OnceCell::new();

    RE.get_or_init(|| Regex::new(r"\((.*?)\)").unwrap())
        .captures_iter(str)
        .map(|capture| {
            let found = capture.get(1).unwrap().as_str();
            if found.contains('(') {
                Err(format!("open bracket unexpected {str}"))
            } else {
                Ok(found)
            }
        })
        .collect()
}

fn detect_xys(brackets: Vec<&str>) -> Result<Vec<XYMark>, String> {
    const RE: OnceCell<Regex> = OnceCell::new();

    brackets
        .iter()
        .map(|line| {
            let captures = RE
                .get_or_init(|| Regex::new(r"^(@?)([-+]?\d+),([-+]?\d+)$").unwrap())
                .captures(line.trim())
                .ok_or_else(|| format!("Cannot parse bracket: value={line}"))?;

            if captures.len() != 4 {
                return Err(format!("Invalid value: value={line}"));
            }

            Ok(XYMark {
                x: captures[2].parse().unwrap(),
                y: captures[3].parse().unwrap(),
                mark: !captures[1].is_empty(),
            })
        })
        .collect()
}

fn create_pattern(xy_marks: Vec<XYMark>) -> _Pattern {
    _Pattern::new(
        xy_marks
            .into_iter()
            .map(|xy_mark| (Coordinate::new(xy_mark.x, xy_mark.y), xy_mark.mark))
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_wrapper(key: &str, value: &str) -> Result<KickPattern, String> {
        create(key.to_string(), value.to_string())
    }

    mod regular {
        use super::*;
        use crate::sfinder_core::{mino::piece::Piece, srs::rotate::Rotate};
        use std::collections::BTreeMap;

        fn assert_wrapper(
            key: &str,
            value: &str,
            (piece, from, to): (Piece, Rotate, Rotate),
            pattern: _Pattern,
        ) {
            assert_eq!(
                create_wrapper(key, value).unwrap(),
                KickPattern::try_new(
                    KickType { piece, from, to },
                    KickPatternType::Fixed { pattern: pattern }
                )
                .unwrap()
            );
        }

        #[test]
        fn fixed1() {
            assert_wrapper(
                "T.NE",
                "(0,0)",
                (Piece::T, Rotate::Spawn, Rotate::Right),
                _Pattern::with_no_privilege_spins(vec![Coordinate::new(0, 0)]),
            );
        }

        #[test]
        fn fixed2() {
            assert_wrapper(
                "S.ES",
                "(1, 1), (2, 2)",
                (Piece::S, Rotate::Right, Rotate::Reverse),
                _Pattern::with_no_privilege_spins(vec![
                    Coordinate::new(1, 1),
                    Coordinate::new(2, 2),
                ]),
            );
        }

        #[test]
        fn fixed3() {
            assert_wrapper(
                "O.SW",
                "( +0 , -0 )( +2 , -2 ) (+3,-3) ",
                (Piece::O, Rotate::Reverse, Rotate::Left),
                _Pattern::with_no_privilege_spins(vec![
                    Coordinate::new(0, 0),
                    Coordinate::new(2, -2),
                    Coordinate::new(3, -3),
                ]),
            );
        }

        #[test]
        fn fixed4_privilege_spins() {
            assert_wrapper(
                "O.SW",
                " (@ -0 , 0 )( -2 , -2 ) (@-3,-3) ",
                (Piece::O, Rotate::Reverse, Rotate::Left),
                _Pattern::new(vec![
                    (Coordinate::new(0, 0), true),
                    (Coordinate::new(-2, -2), false),
                    (Coordinate::new(-3, -3), true),
                ]),
            );
        }

        #[test]
        fn reference1() {
            let referenced = create_wrapper("J.WS", "(0,0)").unwrap();
            let fallback = BTreeMap::from([(referenced.get_kick_type().clone(), referenced)]);
            let reference = create_wrapper("L.SW", "&J.WS").unwrap();
            assert_eq!(
                reference.get_kick_type(),
                &KickType {
                    piece: Piece::L,
                    from: Rotate::Reverse,
                    to: Rotate::Left
                }
            );
            assert_eq!(reference.get_pattern(&BTreeMap::new()), None);
            assert_eq!(
                reference.get_pattern(&fallback),
                Some(&_Pattern::with_no_privilege_spins(vec![Coordinate::new(
                    0, 0
                )]))
            );
        }

        #[test]
        fn alt_rotate() {
            assert_wrapper(
                "T.0R",
                "(1,2)",
                (Piece::T, Rotate::Spawn, Rotate::Right),
                _Pattern::with_no_privilege_spins(vec![Coordinate::new(1, 2)]),
            );
        }
    }

    mod invalid_key {
        use super::*;

        #[test]
        fn empty() {
            assert!(create_wrapper("", "(0,0)").is_err());
        }

        #[test]
        fn invalid_piece() {
            assert!(create_wrapper("K.WS", "(0,0)").is_err());
        }

        #[test]
        fn invalid_mark() {
            assert!(create_wrapper("J_WS", "(0,0)").is_err());
        }

        #[test]
        fn invalid_rotate_from() {
            assert!(create_wrapper("J.2S", "(0,0)").is_err());
        }

        #[test]
        fn invalid_rotate_to() {
            assert!(create_wrapper("J.WL", "(0,0)").is_err());
        }
    }

    mod invalid_value {
        use super::*;

        #[test]
        fn empty() {
            assert!(create_wrapper("O.WS", "").is_err());
        }

        #[test]
        fn no_bracket() {
            assert!(create_wrapper("O.WS", "0,0)(1,1)").is_err());
            assert!(create_wrapper("O.WS", "(0,0(1,1)").is_err());
        }

        #[test]
        fn recursively() {
            assert!(create_wrapper("O.WS", "((0,0))").is_err());
        }

        #[test]
        fn no_comma() {
            assert!(create_wrapper("O.WS", "(00)(1,1)").is_err());
            assert!(create_wrapper("O.WS", "(0 0)(1,1)").is_err());
        }

        #[test]
        fn duplicated_minus() {
            assert!(create_wrapper("O.WS", "(--1,0)").is_err());
        }

        #[test]
        fn invalid_mark() {
            assert!(create_wrapper("O.WS", "(* -1,0)").is_err());
        }

        #[test]
        fn no_ref_mark() {
            assert!(create_wrapper("O.WS", "T.EW").is_err());
        }

        #[test]
        fn reference_to_self() {
            assert!(create_wrapper("O.WS", "&O.WS").is_err());
        }
    }
}
