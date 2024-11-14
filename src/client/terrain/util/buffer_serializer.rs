use renet::Bytes;

#[derive(Debug, PartialEq)]
pub struct RLEToken {
    symbol: u8,
    count: u16,
}

pub fn serialize_buffer(array: Vec<u8>) -> Vec<Bytes> {
    let tokens = tokenize_buffer(array);

    let mut bytes = Vec::<Bytes>::new();
    tokens.iter().for_each(|token| {
        let symbol_bytes = token.symbol.to_le_bytes();
        let count_bytes = token.count.to_le_bytes();
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&symbol_bytes);
        buffer.extend_from_slice(&count_bytes);
        bytes.push(Bytes::from(buffer));
    });

    bytes
}

fn tokenize_buffer(array: Vec<u8>) -> Vec<RLEToken> {
    let mut vec = Vec::<RLEToken>::new();

    let mut last_symbol = array[0];
    let mut count = 1;

    for &element in array.iter().skip(1) {
        if last_symbol == element {
            count += 1;
        } else {
            vec.push(RLEToken {
                count,
                symbol: last_symbol,
            });
            last_symbol = element;
            count = 1;
        }
    }
    vec.push(RLEToken {
        count,
        symbol: last_symbol,
    });

    vec
}

pub fn deserialize_buffer(bytes: Vec<Bytes>) -> Vec<u8> {
    let mut vec = Vec::<u8>::new();

    bytes.iter().for_each(|byte| {
        let symbol = byte[0];
        let count_bytes = &byte[1..3];
        let count = u16::from_le_bytes(count_bytes.try_into().unwrap());

        for _ in 0..count {
            vec.push(symbol);
        }
    });

    vec
}

fn revert_buffer_tokenization(tokens: Vec<RLEToken>) -> Vec<u8> {
    let mut vec = Vec::<u8>::new();

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

    #[test]
    fn test_tokenize_buffer() {
        let array = vec![1, 1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 3];
        let tokens = tokenize_buffer(array);

        let expected_tokens = vec![
            RLEToken { symbol: 1, count: 4 },
            RLEToken { symbol: 2, count: 3 },
            RLEToken { symbol: 3, count: 5 },
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_revert_buffer_tokenization() {
        let tokens = vec![
            RLEToken { symbol: 1, count: 4 },
            RLEToken { symbol: 2, count: 3 },
            RLEToken { symbol: 3, count: 5 },
        ];

        let array = revert_buffer_tokenization(tokens);
        let expected_array = vec![1, 1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 3];

        assert_eq!(array, expected_array);
    }

    #[test]
    fn test_compressed_buffer_is_smaller() {
        let array = vec![1, 1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 3];
        let other_array = array.clone();
        let bytes = serialize_buffer(array);

        let default_bytes = other_array.len() * std::mem::size_of::<u8>();
        let compressed_bytes = bytes.iter().fold(0, |acc, x| acc + x.len());

        assert!(compressed_bytes < default_bytes);
    }

    #[test]
    fn test_serialization_deserialization() {
        let array = vec![1, 1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 3];
        let bytes = serialize_buffer(array.clone());
        let deserialized_array = deserialize_buffer(bytes);
        assert_eq!(array, deserialized_array);
    }
}
