pub static CNUMA_POSSIBLE_SYMBOLS_STRING: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub static CNUMA_POSSIBLE_SYMBOLS: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
pub static BANNED_SYMBOLS: [char; 2] = ['I', 'O'];
pub static BASE_OF_CNUMA_NOTATION: i64 = 36;
pub static FIRST_6_DIGITS_NUMBER: i64 = 100_000;
pub static SIX_DIGITS_CNUM_MAX: i64 = 999_999;
pub static SEVEN_DIGITS_CNUM_MAX: i64 = 3_599_999;
pub static ANTI_OVERFLOW_FOR_BASE_OF_CNUMA: i64 = 600_000_000;
pub static ZERO_ASCII_CODE: u32 = 'A' as u32 - 10;
pub static CNUM_MAXIMUM: i64 = 1_576_782_335;
pub static CNUM_LENGTH: usize = 6;
