use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt::Display;

#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType {
    first_byte: u8,
    second_byte: u8,
    third_byte: u8,
    fourth_byte: u8,
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(ChunkType {
            first_byte: value[0],
            second_byte: value[1],
            third_byte: value[2],
            fourth_byte: value[3],
        })
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        let result = ChunkType {
            first_byte: *bytes.get(0).unwrap(),
            second_byte: *bytes.get(1).unwrap(),
            third_byte: *bytes.get(2).unwrap(),
            fourth_byte: *bytes.get(3).unwrap(),
        };

        if ChunkType::is_valid_byte(&result.first_byte)
        && ChunkType::is_valid_byte(&result.second_byte)
        && ChunkType::is_valid_byte(&result.third_byte)
        && ChunkType::is_valid_byte(&result.fourth_byte) {
            return Ok(result);
        }
        Err("Not a valid Chunk Byte")
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}{}", self.first_byte as char, self.second_byte as char, self.third_byte as char, self.fourth_byte as char)
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        [self.first_byte, self.second_byte, self.third_byte, self.fourth_byte]
    }

    fn is_valid_byte(b: &u8) -> bool {
        ((&65 <= b) && (b <= &90)) || ((&97 <= b) && (b <= &122))
    }

    pub fn is_valid(&self) -> bool {
        ChunkType::is_valid_byte(&self.first_byte)
        && ChunkType::is_valid_byte(&self.second_byte)
        && self.is_reserved_bit_valid()
        && ChunkType::is_valid_byte(&self.fourth_byte)
    }

    fn is_upper(b: &u8) -> bool {
        (&65 <= b) && (b <= &90)
    }

    pub fn is_critical(&self) -> bool {
        ChunkType::is_upper(&self.first_byte)
    }

    pub fn is_public(&self) -> bool {
        ChunkType::is_upper(&self.second_byte)
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        ChunkType::is_upper(&self.third_byte)
    }

    pub fn is_safe_to_copy(&self) -> bool {
        !ChunkType::is_upper(&self.fourth_byte)
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
