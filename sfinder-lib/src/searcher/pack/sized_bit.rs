// Porting note: In normal use only the sizes 4x3 and 5x2 are used, but in testing other sizes are used.
// This is just Plain Old Data, so no need for accessors, just don't mutate them
#[derive(Debug, Clone)]
pub struct SizedBit {
    pub width: u8,
    pub height: u8,
    pub max_bit_digit: u8,
    pub fill_board: u64,
}

impl SizedBit {
    const fn new(width: u8, height: u8) -> Self {
        let max_bit_digit = width * height;
        let fill_board = (1 << max_bit_digit) - 1;

        Self {
            width,
            height,
            max_bit_digit: width * height,
            fill_board,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sized2x4() {
        let sized_bit = SizedBit::new(2, 4);
        assert_eq!(sized_bit.width, 2);
        assert_eq!(sized_bit.height, 4);
        assert_eq!(sized_bit.fill_board, 0b11111111);
        assert_eq!(sized_bit.max_bit_digit, 8);
    }

    #[test]
    fn size2x5() {
        let sized_bit = SizedBit::new(2, 5);
        assert_eq!(sized_bit.width, 2);
        assert_eq!(sized_bit.height, 5);
        assert_eq!(sized_bit.fill_board, 0b1111111111);
        assert_eq!(sized_bit.max_bit_digit, 10);
    }

    #[test]
    fn size3x4() {
        let sized_bit = SizedBit::new(3, 4);
        assert_eq!(sized_bit.width, 3);
        assert_eq!(sized_bit.height, 4);
        assert_eq!(sized_bit.fill_board, 0b111111111111);
        assert_eq!(sized_bit.max_bit_digit, 12);
    }

    #[test]
    fn size3x5() {
        let sized_bit = SizedBit::new(3, 5);
        assert_eq!(sized_bit.width, 3);
        assert_eq!(sized_bit.height, 5);
        assert_eq!(sized_bit.fill_board, 0b111111111111111);
        assert_eq!(sized_bit.max_bit_digit, 15);
    }
}
