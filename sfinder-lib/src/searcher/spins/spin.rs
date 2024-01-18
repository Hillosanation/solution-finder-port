#[derive(PartialEq, PartialOrd)]
pub struct Spin {
    cleared_rows: ClearedRows,
    spin: TSpins,
    name: TSpinNames,
}

impl Spin {
    pub fn new(spin: TSpins, name: TSpinNames, cleared_rows: u8) -> Self {
        debug_assert!(
            !((spin == TSpins::Regular && name == TSpinNames::Neo)
                || (spin == TSpins::Mini && matches!(name, TSpinNames::Iso | TSpinNames::Fin))),
            "invalid spin: spin={spin:?}, name={name:?}"
        );
        Self {
            spin,
            name,
            cleared_rows: ClearedRows::from(cleared_rows),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum TSpins {
    Regular,
    Mini,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum TSpinNames {
    NoName,
    Fin,
    Iso,
    Neo,
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ClearedRows {
    Zero = 0,
    Single,
    Double,
    Triple,
}

impl From<u8> for ClearedRows {
    fn from(value: u8) -> Self {
        match value {
            0 => ClearedRows::Zero,
            1 => ClearedRows::Single,
            2 => ClearedRows::Double,
            3 => ClearedRows::Triple,
            _ => panic!("invalid number of cleared rows: {value}"),
        }
    }
}
