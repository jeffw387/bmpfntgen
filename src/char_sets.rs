use std::str::FromStr;

const ENGLISH_LOWER_CASE: &str = "abcdefghijklmnopqrstuvwxyz";
const ENGLISH_UPPER_CASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const COMMON_SYMBOLS: &str = r###"`~!@#$%^&*()-_=+[]{}|;:'",.<>?\/"###;
const TEST_SET: &str =  "Aa";

#[derive(Debug)]
pub enum CharSets {
    EnglishLowerCase,
    EnglishUpperCase,
    Digits,
    CommonSymbols,
    TestSet,
}

impl FromStr for CharSets {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<CharSets, Self::Err> {
        match s {
            "EnglishLowerCase" => Ok(CharSets::EnglishLowerCase),
            "EnglishUpperCase" => Ok(CharSets::EnglishUpperCase),
            "Digits" => Ok(CharSets::Digits),
            "CommonSymbols" => Ok(CharSets::CommonSymbols),
            "TestSet" => Ok(CharSets::TestSet),
            _ => Err("No matching charset found!"),
        }
    }
}

impl Into<&'static str> for CharSets {
    fn into(self: Self) -> &'static str {
        match self {
            CharSets::EnglishLowerCase => ENGLISH_LOWER_CASE,
            CharSets::EnglishUpperCase => ENGLISH_UPPER_CASE,
            CharSets::Digits => DIGITS,
            CharSets::CommonSymbols => COMMON_SYMBOLS,
            CharSets::TestSet => TEST_SET,
        }
    }
}
