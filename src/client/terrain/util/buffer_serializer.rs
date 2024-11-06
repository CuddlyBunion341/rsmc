use renet::Bytes;

pub struct RLEToken {
    symbol: i32,
    count: i32,
}

pub fn serialize_buffer(array: Vec<i32>) -> Vec<Bytes> {
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

fn tokenize_buffer(array: Vec<i32>) -> Vec<RLEToken> {
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

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_tokenize_buffer() {
        let array = vec![1, 1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 3];
        let tokens = tokenize_buffer(array);

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].symbol, 1);
        assert_eq!(tokens[0].count, 4);
        assert_eq!(tokens[1].symbol, 2);
        assert_eq!(tokens[1].count, 3);
        assert_eq!(tokens[2].symbol, 3);
        assert_eq!(tokens[2].count, 5);
    }
}
