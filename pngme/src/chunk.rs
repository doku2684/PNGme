use crate::chunk_type::ChunkType;
use std::string::FromUtf8Error;
use std::fmt::Display;
use crc::crc32::checksum_ieee;

#[derive(Clone, Debug)]
pub struct Chunk {
    data_length: u32,
    chunk_type: ChunkType,
    message_bytes: Vec<u8>,
    crc: u32,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;

    fn try_from(sequence: &[u8]) -> Result<Self, Self::Error> {
        let data_length_bytes: [u8; 4] = sequence[0..4].try_into().expect("invalid sequence");
        let data_length: u32 = u32::from_be_bytes(data_length_bytes);

        let chunk_type_bytes: [u8; 4] = sequence[4..8].try_into().expect("invalid sequence");
        let chunk_type: ChunkType = ChunkType::try_from(chunk_type_bytes).unwrap();

        let mut message_bytes = sequence[8..].to_vec();

        let crc_bytes: [u8; 4] = message_bytes.split_off(data_length as usize).try_into().expect("invalid sequence");
        let crc: u32 = u32::from_be_bytes(crc_bytes);

        let check_bytes: &[u8] = &sequence[4..(sequence.len()-4)];
        
        if checksum_ieee(check_bytes) != crc {
            return Err("checksum failed");
        }

        Ok(Chunk {data_length, chunk_type, message_bytes, crc})
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.data_length, self.chunk_type, self.crc)
    }
}

impl Chunk {
    pub fn length(&self) -> u32 {
        self.data_length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.message_bytes.as_slice()
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> std::result::Result<String, FromUtf8Error> {
        String::from_utf8(self.message_bytes.clone())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.data_length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.to_string().as_bytes().iter())
            .chain(self.message_bytes.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect() 
    }

    pub fn new(chunk_type: ChunkType, message_bytes: Vec<u8>) -> Self {
        let check_bytes:&[u8] = &[chunk_type.bytes().as_slice(), message_bytes.as_slice()].concat();

        Chunk {
            data_length: message_bytes.len() as u32,
            chunk_type,
            message_bytes,
            crc: checksum_ieee(check_bytes),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}
