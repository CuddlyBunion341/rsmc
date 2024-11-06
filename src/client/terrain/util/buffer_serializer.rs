use renet::Bytes;
use std::convert::TryFrom;
use rsmc::BlockId;

#[derive(Debug, PartialEq)]
pub struct RLEToken {
    symbol: BlockId,
    count: i32,
}

pub fn serialize_buffer(array: Vec<BlockId>) -> Vec<Bytes> {
    let tokens = tokenize_buffer(array);

    let mut bytes = Vec::<Bytes>::new();
    tokens.iter().for_each(|token| {
        let symbol_bytes = (token.symbol as i32).to_le_bytes();
        let count_bytes = token.count.to_le_bytes();
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&symbol_bytes);
        buffer.extend_from_slice(&count_bytes);
        bytes.push(Bytes::from(buffer));
    });

    bytes
}

fn tokenize_buffer(array: Vec<BlockId>) -> Vec<RLEToken> {
    let mut vec = Vec::<RLEToken>::new();

    let mut last_symbol = array[0];
    let mut count = 1;

    for element in array.iter().skip(1) {
        if last_symbol == *element {
            count += 1;
        } else {
            vec.push(RLEToken {
                count,
                symbol: last_symbol,
            });
            last_symbol = *element;
            count = 1;
        }
    }
    vec.push(RLEToken {
        count,
        symbol: last_symbol,
    });

    vec
}

pub fn deserialize_buffer(bytes: Vec<Bytes>) -> Vec<BlockId> {
    let mut vec = Vec::<BlockId>::new();

    bytes.iter().for_each(|byte| {
        let symbol_bytes = &byte[0..4];
        let count_bytes = &byte[4..8];
        let symbol = BlockId::try_from(i32::from_le_bytes(symbol_bytes.try_into().unwrap())).unwrap();
        let count = i32::from_le_bytes(count_bytes.try_into().unwrap());

        for _ in 0..count {
            vec.push(symbol);
        }
    });

    vec
}

fn revert_buffer_tokenization(tokens: Vec<RLEToken>) -> Vec<BlockId> {
    let mut vec = Vec::<BlockId>::new();

    tokens.iter().for_each(|token| {
        for _ in 0..token.count {
            vec.push(token.symbol);
        }
    });

    vec
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use rsmc::BlockId;

    #[test]
    fn test_tokenize_buffer() {
        let array = vec![
            BlockId::Air, BlockId::Air, BlockId::Air, BlockId::Air,
            BlockId::Grass, BlockId::Grass, BlockId::Grass,
            BlockId::Dirt, BlockId::Dirt, BlockId::Dirt, BlockId::Dirt, BlockId::Dirt,
        ];
        let tokens = tokenize_buffer(array);

        let expected_tokens = vec![
            RLEToken {
                symbol: BlockId::Air,
                count: 4,
            },
            RLEToken {
                symbol: BlockId::Grass,
                count: 3,
            },
            RLEToken {
                symbol: BlockId::Dirt,
                count: 5,
            },
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_revert_buffer_tokenization() {
        let tokens = vec![
            RLEToken {
                symbol: BlockId::Air,
                count: 4,
            },
            RLEToken {
                symbol: BlockId::Grass,
                count: 3,
            },
            RLEToken {
                symbol: BlockId::Dirt,
                count: 5,
            },
        ];

        let array = revert_buffer_tokenization(tokens);
        let expected_array = vec![
            BlockId::Air, BlockId::Air, BlockId::Air, BlockId::Air,
            BlockId::Grass, BlockId::Grass, BlockId::Grass,
            BlockId::Dirt, BlockId::Dirt, BlockId::Dirt, BlockId::Dirt, BlockId::Dirt,
        ];

        assert_eq!(array, expected_array);
    }

    #[test]
    fn test_compressed_buffer_is_smaller() {
        let array = vec![
            BlockId::Air, BlockId::Air, BlockId::Air, BlockId::Air,
            BlockId::Grass, BlockId::Grass, BlockId::Grass,
            BlockId::Dirt, BlockId::Dirt, BlockId::Dirt, BlockId::Dirt, BlockId::Dirt,
        ];
        let other_array = array.clone();
        let bytes = serialize_buffer(array);

        let default_bytes = other_array.len() * std::mem::size_of::<BlockId>();
        let compressed_bytes = bytes.iter().fold(0, |acc, x| acc + x.len());

        assert!(compressed_bytes < default_bytes);
    }

    #[test]
    fn test_serialization_deserialization() {
        let array = vec![
            BlockId::Air, BlockId::Air, BlockId::Air, BlockId::Air,
            BlockId::Grass, BlockId::Grass, BlockId::Grass,
            BlockId::Dirt, BlockId::Dirt, BlockId::Dirt, BlockId::Dirt, BlockId::Dirt,
        ];
        let bytes = serialize_buffer(array.clone());
        let deserialized_array = deserialize_buffer(bytes);
        assert_eq!(array, deserialized_array);
    }
}
