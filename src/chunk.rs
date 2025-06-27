#![allow(unused_variables)]

use std::str;
use crate::chunk_type::ChunkType;

#[derive(PartialEq, Eq, Debug)]
pub struct Chunk{
    pub data: Vec<u8>,
    pub chunk_type: ChunkType,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;
    fn try_from(chunk_bytes: &[u8]) -> Result<Self, Self::Error> {
        if chunk_bytes.len() < 12 {
            return Err("Chunk bytes too short");
        }
        let chunk_type_bytes = [chunk_bytes[4], chunk_bytes[5], chunk_bytes[6], chunk_bytes[7]];
        let crc_bytes = &chunk_bytes[chunk_bytes.len() - 4..];
        let crc_array = [crc_bytes[0], crc_bytes[1], crc_bytes[2], crc_bytes[3]];
        let crc = u32::from_be_bytes(crc_array);
        let crc_calculator = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let calculated_crc = crc_calculator.checksum(&chunk_bytes[4..chunk_bytes.len()-4]);
        if calculated_crc == crc {
        Ok(Chunk { data: chunk_bytes[8..chunk_bytes.len() - 4].to_vec(), chunk_type: ChunkType::try_from(chunk_type_bytes)?})
        }
        else{
            Err("CRC Mismatch")
        }
    }
}

impl Chunk {
    // Constructor method
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        // Build a chunk from scratch
        Chunk { data: data, chunk_type: chunk_type }
    }
    
    // Getter methods
    pub fn length(&self) -> u32 {
        // Return the data length (first 4 bytes)
        self.data.len() as u32
    }
    
    pub fn chunk_type(&self) -> &ChunkType {
        // Return the chunk type (bytes 4-7)
        &self.chunk_type
    }
    
    pub fn data(&self) -> &[u8] {
        // Return just the data portion (bytes 8 to length-4)
        return &self.data
    }
    
    pub fn crc(&self) -> u32 {
        // Return the CRC (last 4 bytes)
        let mut combined = Vec::new();
        combined.extend_from_slice(&self.chunk_type.bytes());
        combined.extend_from_slice(&self.data);
        let crc_calculator = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let calculated_crc = crc_calculator.checksum(&combined);
        calculated_crc
    }
    
    pub fn data_as_string(&self) -> Result<String, std::str::Utf8Error> {
        // Convert the data bytes to a string
        str::from_utf8(&self.data)
        .map(|s| s.to_string())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length()
        .to_be_bytes()
        .iter()
        .chain(self.chunk_type.bytes().iter())  // Use .bytes() method
        .chain(self.data.iter())
        .chain(self.crc().to_be_bytes().iter())
        .copied()
        .collect()  
    }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // What should you write here?
        write!(f, "{}", self.data_as_string().unwrap_or_else(|_| "Invalid UTF-8".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

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
    fn test_chunk () {
        let chunk = testing_chunk();
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
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
