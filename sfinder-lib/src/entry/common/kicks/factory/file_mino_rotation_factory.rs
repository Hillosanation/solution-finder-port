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
                .ok_or_else(|| format!("Cannot parse line as key value pair: line={line}"))?;

            // multi-line values are not supported
            properties.insert(key.to_string(), value.to_string());
        }

        Ok(properties)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::datastore::coordinate::Coordinate,
        entry::common::kicks::factory::srs_mino_rotation_factory,
        sfinder_core::{
            field::field_factory,
            mino::{mino_factory::MinoFactory, piece::Piece},
            srs::{rotate::Rotate, rotate_direction::RotateDirection},
        },
    };

    fn get_file_path(name: &str) -> PathBuf {
        let mut d = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        d.push("test_resources/kicks");
        d.push(name);
        d.set_extension("properties");
        d
    }

    #[test]
    fn load_no_kick_90() {
        let field = field_factory::create_small_field();
        let mino_factory = MinoFactory::new();
        let mino_rotation = create(get_file_path("nokick90")).unwrap();

        assert!(!mino_rotation.supports_180());

        // rotate cw
        let before = mino_factory.get(Piece::T, Rotate::Spawn);
        let after = mino_factory.get(Piece::T, Rotate::Right);

        assert_eq!(
            mino_rotation.get_kicks(&field, before, after, 1, 1, RotateDirection::Clockwise),
            Some(Coordinate::new(0, 0))
        );

        assert_eq!(
            mino_rotation.get_kicks(&field, before, after, 1, 0, RotateDirection::Clockwise),
            None
        );

        // rotate 180
        let before = mino_factory.get(Piece::T, Rotate::Spawn);
        let after = mino_factory.get(Piece::T, Rotate::Reverse);

        assert_eq!(
            mino_rotation.get_kicks(&field, before, after, 1, 1, RotateDirection::Rotate180),
            None
        );
    }

    #[test]
    fn load_no_kick_180() {
        let field = field_factory::create_small_field();
        let mino_factory = MinoFactory::new();
        let mino_rotation = create(get_file_path("nokick180")).unwrap();

        assert!(mino_rotation.supports_180());

        // rotate cw
        let before = mino_factory.get(Piece::T, Rotate::Spawn);
        let after = mino_factory.get(Piece::T, Rotate::Right);

        assert_eq!(
            mino_rotation.get_kicks(&field, before, after, 1, 1, RotateDirection::Clockwise),
            Some(Coordinate::new(0, 0))
        );

        assert_eq!(
            mino_rotation.get_kicks(&field, before, after, 1, 0, RotateDirection::Clockwise),
            None
        );

        // rotate 180
        let before = mino_factory.get(Piece::T, Rotate::Spawn);
        let after = mino_factory.get(Piece::T, Rotate::Reverse);

        assert_eq!(
            mino_rotation.get_kicks(&field, before, after, 1, 1, RotateDirection::Rotate180),
            Some(Coordinate::new(0, 0))
        );

        assert_eq!(
            mino_rotation.get_kicks(&field, before, after, 1, 0, RotateDirection::Rotate180),
            None
        );
    }

    #[test]
    fn load_srs() {
        let mino_factory = MinoFactory::new();
        let mino_rotation = create(get_file_path("srs")).unwrap();
        let default_mino_rotation = srs_mino_rotation_factory::create();

        assert!(!mino_rotation.supports_180());

        for &piece in Piece::value_list() {
            for &rotate in Rotate::value_list() {
                let mino = mino_factory.get(piece, rotate);
                for &direction in RotateDirection::values_no_180() {
                    assert_eq!(
                        mino_rotation.get_patterns_from(mino, direction),
                        default_mino_rotation.get_patterns_from(mino, direction)
                    );
                }
            }
        }
    }

    #[test]
    fn load_two_levels_of_reference() {
        let mino_factory = MinoFactory::new();
        let mino_rotation = create(get_file_path("two_levels_of_reference")).unwrap();
        let default_mino_rotation = srs_mino_rotation_factory::create();

        assert!(!mino_rotation.supports_180());

        for &piece in Piece::value_list() {
            for &rotate in Rotate::value_list() {
                let mino = mino_factory.get(piece, rotate);
                for &direction in RotateDirection::values_no_180() {
                    assert_eq!(
                        mino_rotation.get_patterns_from(mino, direction),
                        default_mino_rotation.get_patterns_from(mino, direction)
                    );
                }
            }
        }
    }

    #[test]
    fn load_missing_i_en() {
        let path = get_file_path("missing_I_EN");
        assert!(create(path).is_err());
    }

    #[test]
    fn load_surplus_i_en() {
        let path = get_file_path("surplus_I_EW");
        assert!(create(path).is_err());
    }
}
