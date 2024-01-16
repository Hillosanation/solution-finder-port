use std::cell::OnceCell;

use crate::{
    common::datastore::coordinate::Coordinate, entry::common::kicks::kick_pattern::KickPatternType,
    sfinder_core::srs::pattern::Pattern,
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
    let kick_pattern_type = if trimmed_key.starts_with('&') {
        KickPatternType::Referenced {
            reference_kick_type: parse_to_kick_type(trimmed_value.trim_start_matches('&'))?,
        }
    } else {
        KickPatternType::Fixed {
            pattern: parse_to_pattern(trimmed_value)?,
        }
    };

    Ok(KickPattern::new(kick_type, kick_pattern_type))
}

fn parse_to_kick_type(str: &str) -> Result<KickType, String> {
    const RE: OnceCell<Regex> = OnceCell::new();
    let capture = &RE
        .get_or_init(|| Regex::new(r"[TIOLJSZ].[NEWSLR02]{2}").unwrap())
        .find(&str)
        .unwrap()
        .as_str();

    Ok(KickType {
        piece: capture[0..1].parse().unwrap(),
        from: capture[2..3].parse().unwrap(),
        to: capture[3..4].parse().unwrap(),
    })
}

fn parse_to_pattern(str: String) -> Result<Pattern, String> {
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
                .unwrap();

            if captures.len() != 4 {
                return Err(format!("Invalid value: value={line}"));
            }

            Ok(XYMark {
                x: captures[1].parse().unwrap(),
                y: captures[2].parse().unwrap(),
                mark: !captures[3].is_empty(),
            })
        })
        .collect()
}

fn create_pattern(xy_marks: Vec<XYMark>) -> Pattern {
    let len = xy_marks.len();
    let mut offsets = Vec::with_capacity(len);
    let mut privilege_spins = Vec::with_capacity(len);

    for xy_mark in xy_marks {
        offsets.push(Coordinate::new(xy_mark.x, xy_mark.y));
        privilege_spins.push(xy_mark.mark);
    }

    Pattern::new(offsets, privilege_spins)
}
