use super::mino_rotation_supplier;
use crate::{
    entry::common::kicks::{kick_pattern_interpreter, kick_patterns::KickPatterns},
    sfinder_core::srs::mino_rotation::MinoRotation,
};
use std::path::PathBuf;

// Porting note: replaces load
pub fn create(path: PathBuf) -> Result<Box<dyn MinoRotation>, String> {
    let kick_pattern_list = properties_parser::parse_file(path)?
        .into_iter()
        .map(|(key, value)| kick_pattern_interpreter::create(key, value))
        .collect::<Result<Vec<_>, _>>()?;
    let kick_patterns = KickPatterns::new(kick_pattern_list);
    mino_rotation_supplier::create_new_rotation(kick_patterns)
}

mod properties_parser {
    //! Partially parses a properties file. I did not implement the full specification of a properties file
    //! in Java, because there was no need to do so in the existing properties files.

    use super::*;
    use std::collections::HashMap;

    pub type Properties = HashMap<String, String>;

    pub fn parse_file(path: PathBuf) -> Result<Properties, String> {
        assert_eq!(path.extension().unwrap(), "properties");
        let buf = std::fs::read(path).map_err(|e| e.to_string())?;
        let str = String::from_utf8(buf).map_err(|e| e.to_string())?;
        parse(str)
    }

    fn parse(str: String) -> Result<Properties, String> {
        let mut properties = HashMap::new();

        for line in str.lines() {
            if line.starts_with(&['#', '!']) {
                continue;
            }
            let trim = line.replace(&[' ', '\t', '\u{000C}'], "");
            if trim.is_empty() {
                continue;
            }

            // escape sequences are not supported
            let (key, value) = trim
                .split_once(&[':', '='])
                .ok_or_else(|| format!("Cannot parse line: line={line}"))?;

            // multi-line values are not supported
            properties.insert(key.to_string(), value.to_string());
        }

        Ok(properties)
    }
}
