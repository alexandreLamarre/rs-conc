use std::convert::TryInto;

/// Converts a byte slice to one of size 4.
fn convert_slice_to_fixed(arr: &[u8]) -> [u8; 4] {
    arr.try_into().expect("Slice with incorrect length")
}

/// Struct to implement a chunk type encoding for PNGs
/// 4-byte consisting of only uppercase and lowercase ASCII letters
/// (A-Z and a-z, or 65-90 and 97-122 decimal)
#[derive(PartialEq, Debug)]
struct ChunkType {
    _container: [u8; 4],
}

/// methods
impl ChunkType {
    /// Create from static byte array
    pub fn try_from(arr: [u8; 4]) -> Result<ChunkType, String> {
        for &byte_val in arr.iter() {
            // Check valid ascii value
            if byte_val < 65 || (byte_val > 90 && byte_val < 97) || byte_val > 122 {
                return Err(
                    "Chunk type encoding must be in ascii lowercase/upper case (65-90/97-122)"
                        .to_string(),
                );
            }
        }
        let c = ChunkType { _container: arr };
        Ok(c)
    }
    /// Create from &str slice
    pub fn from_str(input_str: &str) -> Result<ChunkType, String> {
        let res: ChunkType = ChunkType::try_from(convert_slice_to_fixed(input_str.as_bytes()))?;

        Ok(res)
    }
    /// Returns the actual bytes of the Chunk Type
    pub fn bytes(&self) -> [u8; 4] {
        self._container
    }

    /// Checks whether or not the chunk is critical to decoding the image
    ///     Checks the Ancillary bit of the first byte : bit 5 of the first byte
    pub fn is_critical(&self) -> bool {
        (self._container[0] & (1 << 5)) == 0
    }

    /// Checks whether or not the chunk is public to decoding the image
    /// private chunks can be used to encode other information
    ///     Checks the Ancillary bit of the second byte : bit 5 of the second byte
    pub fn is_public(&self) -> bool {
        (self._container[1] & (1 << 5)) == 0
    }

    /// Checks whether or not the chunk is reserved for future expansion
    /// (Some future version of PNG) could use this for something important
    pub fn is_reserved_bit_valid(&self) -> bool {
        (self._container[2] & (1 << 5)) == 0
    }

    /// Checks whther it is safe to copy. (Needed by PNG editors, but not decoders)
    pub fn is_safe_to_copy(&self) -> bool {
        (self._container[3] & (1 << 5)) != 0
    }

    /// Checks if the chunk type is valid for the PNG format
    /// TODO: Not sure what the full conditions for this is.
    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
}

impl ToString for ChunkType {
    fn to_string(&self) -> String {
        let s = match String::from_utf8(self._container.to_vec()) {
            Ok(v) => v,
            Err(e) => panic!("Invalid byte sequence for UTF-8 sequence {}", e),
        };
        s
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
        println!("{:?}", chunk);
        println!("{:?}", "RuSt".bytes());
        println!("{:?}", chunk.is_safe_to_copy());
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        println!("{:?}", chunk);
        println!("{:?}", "RuST".bytes());
        println!("{:?}", chunk.is_safe_to_copy());
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

    // #[test]
    // pub fn test_chunk_type_trait_impls() {
    //     let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
    //     let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
    //     let _chunk_string = format!("{}", chunk_type_1);
    //     let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    // }
}
