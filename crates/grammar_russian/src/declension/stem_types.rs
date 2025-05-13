#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnyStemType {
    Type1 = 1,
    Type2 = 2,
    Type3 = 3,
    Type4 = 4,
    Type5 = 5,
    Type6 = 6,
    Type7 = 7,
    Type8 = 8,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NounStemType {
    Type1 = 1,
    Type2 = 2,
    Type3 = 3,
    Type4 = 4,
    Type5 = 5,
    Type6 = 6,
    Type7 = 7,
    Type8 = 8,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AdjectiveStemType {
    Type1 = 1,
    Type2 = 2,
    Type3 = 3,
    Type4 = 4,
    Type5 = 5,
    Type6 = 6,
    Type7 = 7,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PronounStemType {
    Type1 = 1,
    Type2 = 2,
    Type4 = 4,
    Type6 = 6,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AnyStemTypeError;
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NounStemTypeError;
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AdjectiveStemTypeError;
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PronounStemTypeError;

impl std::fmt::Display for AnyStemTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "a declension can only have stem types 1 through 8".fmt(f)
    }
}
impl std::fmt::Display for NounStemTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "noun declension can only have stem types 1 through 8".fmt(f)
    }
}
impl std::fmt::Display for AdjectiveStemTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "adjective declension can only have stem types 1 through 7".fmt(f)
    }
}
impl std::fmt::Display for PronounStemTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "pronoun declension can only have stem types 1, 2, 4 and 6".fmt(f)
    }
}
impl std::error::Error for AnyStemTypeError {}
impl std::error::Error for NounStemTypeError {}
impl std::error::Error for AdjectiveStemTypeError {}
impl std::error::Error for PronounStemTypeError {}

impl TryFrom<u8> for AnyStemType {
    type Error = AnyStemTypeError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::Type1,
            2 => Self::Type2,
            3 => Self::Type3,
            4 => Self::Type4,
            5 => Self::Type5,
            6 => Self::Type6,
            7 => Self::Type7,
            8 => Self::Type8,
            _ => return Err(Self::Error {}),
        })
    }
}
impl TryFrom<u8> for NounStemType {
    type Error = NounStemTypeError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::Type1,
            2 => Self::Type2,
            3 => Self::Type3,
            4 => Self::Type4,
            5 => Self::Type5,
            6 => Self::Type6,
            7 => Self::Type7,
            8 => Self::Type8,
            _ => return Err(Self::Error {}),
        })
    }
}
impl TryFrom<u8> for AdjectiveStemType {
    type Error = AdjectiveStemTypeError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::Type1,
            2 => Self::Type2,
            3 => Self::Type3,
            4 => Self::Type4,
            5 => Self::Type5,
            6 => Self::Type6,
            7 => Self::Type7,
            _ => return Err(Self::Error {}),
        })
    }
}
impl TryFrom<u8> for PronounStemType {
    type Error = PronounStemTypeError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::Type1,
            2 => Self::Type2,
            4 => Self::Type4,
            6 => Self::Type6,
            _ => return Err(Self::Error {}),
        })
    }
}

impl From<NounStemType> for AnyStemType {
    fn from(value: NounStemType) -> Self {
        (value as u8).try_into().unwrap()
    }
}
impl From<AdjectiveStemType> for AnyStemType {
    fn from(value: AdjectiveStemType) -> Self {
        (value as u8).try_into().unwrap()
    }
}
impl From<PronounStemType> for AnyStemType {
    fn from(value: PronounStemType) -> Self {
        (value as u8).try_into().unwrap()
    }
}

impl From<AnyStemType> for NounStemType {
    fn from(value: AnyStemType) -> Self {
        (value as u8).try_into().unwrap()
    }
}
impl From<AdjectiveStemType> for NounStemType {
    fn from(value: AdjectiveStemType) -> Self {
        (value as u8).try_into().unwrap()
    }
}
impl From<PronounStemType> for NounStemType {
    fn from(value: PronounStemType) -> Self {
        (value as u8).try_into().unwrap()
    }
}

impl TryFrom<AnyStemType> for AdjectiveStemType {
    type Error = AdjectiveStemTypeError;
    fn try_from(value: AnyStemType) -> Result<Self, Self::Error> {
        (value as u8).try_into()
    }
}
impl TryFrom<NounStemType> for AdjectiveStemType {
    type Error = AdjectiveStemTypeError;
    fn try_from(value: NounStemType) -> Result<Self, Self::Error> {
        (value as u8).try_into()
    }
}
impl From<PronounStemType> for AdjectiveStemType {
    fn from(value: PronounStemType) -> Self {
        (value as u8).try_into().unwrap()
    }
}

impl TryFrom<AnyStemType> for PronounStemType {
    type Error = PronounStemTypeError;
    fn try_from(value: AnyStemType) -> Result<Self, Self::Error> {
        (value as u8).try_into()
    }
}
impl TryFrom<NounStemType> for PronounStemType {
    type Error = PronounStemTypeError;
    fn try_from(value: NounStemType) -> Result<Self, Self::Error> {
        (value as u8).try_into()
    }
}
impl TryFrom<AdjectiveStemType> for PronounStemType {
    type Error = PronounStemTypeError;
    fn try_from(value: AdjectiveStemType) -> Result<Self, Self::Error> {
        (value as u8).try_into()
    }
}

impl std::fmt::Display for AnyStemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ((b'0' + *self as u8) as char).fmt(f)
    }
}
impl std::fmt::Display for NounStemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ((b'0' + *self as u8) as char).fmt(f)
    }
}
impl std::fmt::Display for AdjectiveStemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ((b'0' + *self as u8) as char).fmt(f)
    }
}
impl std::fmt::Display for PronounStemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ((b'0' + *self as u8) as char).fmt(f)
    }
}

impl std::str::FromStr for AnyStemType {
    type Err = AnyStemTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_bytes() {
            [ch @ _] => Self::try_from(*ch - b'0'),
            _ => Err(Self::Err {}),
        }
    }
}
impl std::str::FromStr for NounStemType {
    type Err = NounStemTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_bytes() {
            [ch @ _] => Self::try_from(*ch - b'0'),
            _ => Err(Self::Err {}),
        }
    }
}
impl std::str::FromStr for AdjectiveStemType {
    type Err = AdjectiveStemTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_bytes() {
            [ch @ _] => Self::try_from(*ch - b'0'),
            _ => Err(Self::Err {}),
        }
    }
}
impl std::str::FromStr for PronounStemType {
    type Err = PronounStemTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_bytes() {
            [ch @ _] => Self::try_from(*ch - b'0'),
            _ => Err(Self::Err {}),
        }
    }
}
