#[derive(Debug, PartialEq)]
pub struct RLEToken {
    symbol: u8,
    count: u16,
}

const TOKEN_BYTE_COUNT: usize = 3;
const SYMBOL_OFFSET: usize = 0;
const COUNT_OFFSET: usize = 1;
const COUNT_LENGTH: usize = 2;

pub fn serialize_buffer(array: Vec<u8>) -> Vec<u8> {
    let tokens = tokenize_buffer(array);

    let mut bytes = Vec::<u8>::new();
    tokens.iter().for_each(|token| {
        let symbol_bytes = token.symbol.to_le_bytes();
        let count_bytes = token.count.to_le_bytes();
        bytes.extend_from_slice(&symbol_bytes);
        bytes.extend_from_slice(&count_bytes);
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

pub fn deserialize_buffer(bytes: &[u8]) -> Vec<u8> {
    let mut vec = Vec::<u8>::new();

    let mut i = 0;
    while i < bytes.len() {
        let symbol = bytes[i + SYMBOL_OFFSET];
        let count_bytes = &bytes[i + COUNT_OFFSET..i + COUNT_OFFSET + COUNT_LENGTH];
        let count = u16::from_le_bytes(count_bytes.try_into().unwrap());

        for _ in 0..count {
            vec.push(symbol);
        }

        i += TOKEN_BYTE_COUNT;
    }

    vec
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn revert_buffer_tokenization(tokens: Vec<RLEToken>) -> Vec<u8> {
        let mut vec = Vec::<u8>::new();

        tokens.iter().for_each(|token| {
            for _ in 0..token.count {
                vec.push(token.symbol);
            }
        });

        vec
    }

    #[test]
    fn test_tokenize_buffer() {
        #[rustfmt::skip]
        let array = vec![1, 1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 3];
        let tokens = tokenize_buffer(array);

        #[rustfmt::skip]
        let expected_tokens = vec![
            RLEToken { symbol: 1, count: 4 },
            RLEToken { symbol: 2, count: 3 },
            RLEToken { symbol: 3, count: 5 },
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_revert_buffer_tokenization() {
        #[rustfmt::skip]
        let tokens = vec![
            RLEToken { symbol: 1, count: 4 },
            RLEToken { symbol: 2, count: 3 },
            RLEToken { symbol: 3, count: 5 },
        ];

        let array = revert_buffer_tokenization(tokens);
        #[rustfmt::skip]
        let expected_array = vec![1, 1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 3];

        assert_eq!(array, expected_array);
    }

    #[test]
    fn test_compressed_buffer_is_smaller() {
        #[rustfmt::skip]
        let array = vec![1, 1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 3];
        let other_array = array.clone();
        let bytes = serialize_buffer(array);

        let default_bytes = other_array.len() * std::mem::size_of::<u8>();
        let compressed_bytes = bytes.len();

        assert!(compressed_bytes < default_bytes);
    }

    #[test]
    fn test_serialization_deserialization() {
        #[rustfmt::skip]
        let array = vec![1, 1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 3];
        let bytes = serialize_buffer(array.clone());
        let deserialized_array = deserialize_buffer(&bytes);
        assert_eq!(array, deserialized_array);
    }
}
