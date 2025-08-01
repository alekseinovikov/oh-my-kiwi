use num_bigint::BigInt;
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Types {
    SimpleString(String),
    SimpleError(String),
    Integer(i64),
    BulkString(String),
    Array(Vec<Types>),
    Null,
    Boolean(bool),
    Double(OrderedFloat<f64>),
    BigNumber(BigInt),
    BulkError(String),
    Map(BTreeMap<Types, Types>),
    Set(Vec<Types>),
}

impl Types {
    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        match self {
            Types::SimpleString(payload) => simple_string_to_bytes(payload),
            Types::SimpleError(payload) => simple_error_to_bytes(payload),
            Types::Integer(value) => integer_to_bytes(value),
            Types::BulkString(payload) => bulk_string_to_bytes(payload),
            Types::Array(array) => array_to_bytes(array),
            Types::Null => null_to_bytes(),
            Types::Boolean(value) => boolean_to_bytes(value),
            Types::Double(value) => double_to_bytes(*value),
            Types::BigNumber(value) => big_number_to_bytes(value),
            Types::BulkError(payload) => bulk_error_to_bytes(payload),
            Types::Map(map) => map_to_bytes(map),
            Types::Set(set) => set_to_bytes(set),
        }
    }
}

const CRLF: &[u8] = b"\r\n";
const CRLF_LEN: usize = 2;

fn simple_string_to_bytes(payload: &str) -> Vec<u8> {
    let mut result = Vec::with_capacity(payload.len() + CRLF_LEN + 1);
    result.extend_from_slice(b"+");
    result.extend_from_slice(payload.as_bytes());
    result.extend_from_slice(CRLF);
    result
}

fn simple_error_to_bytes(payload: &str) -> Vec<u8> {
    let mut result = Vec::with_capacity(payload.len() + CRLF_LEN + 1);
    result.extend_from_slice(b"-");
    result.extend_from_slice(payload.as_bytes());
    result.extend_from_slice(CRLF);
    result
}

fn integer_to_bytes(value: &i64) -> Vec<u8> {
    // allocate 16 bytes as default
    let mut result = Vec::with_capacity(CRLF_LEN + 1 + 13);
    result.extend_from_slice(b":");
    result.extend_from_slice(value.to_string().as_bytes());
    result.extend_from_slice(CRLF);
    result
}

fn bulk_string_to_bytes(payload: &str) -> Vec<u8> {
    // 2CRLF + len + size number
    let mut result = Vec::with_capacity(payload.len() + CRLF_LEN * 2 + 1 + 13);
    result.extend_from_slice(b"$");
    result.extend_from_slice(payload.len().to_string().as_bytes());
    result.extend_from_slice(CRLF);
    result.extend_from_slice(payload.as_bytes());
    result.extend_from_slice(CRLF);
    result
}

fn array_to_bytes(arr: &Vec<Types>) -> Vec<u8> {
    let mut result = Vec::with_capacity(CRLF_LEN + 1 + 13);
    result.extend_from_slice(b"*");
    if arr.is_empty() {
        result.extend_from_slice(b"0");
        result.extend_from_slice(CRLF);
        return result;
    }

    let len = arr.len();
    let len_str = len.to_string();
    let len_bytes = len_str.as_bytes();
    result.extend_from_slice(len_bytes);
    result.extend_from_slice(CRLF);

    let elems_bytes: Vec<u8> = arr.iter().flat_map(|elem| elem.to_bytes()).collect();
    result.extend_from_slice(&elems_bytes);
    result
}

fn null_to_bytes() -> Vec<u8> {
    let mut result = Vec::with_capacity(CRLF_LEN + 1);
    result.extend_from_slice(b"_");
    result.extend_from_slice(CRLF);
    result
}

fn boolean_to_bytes(value: &bool) -> Vec<u8> {
    let mut result = Vec::with_capacity(CRLF_LEN + 2);
    result.extend_from_slice(if *value { b"#t" } else { b"#f" });
    result.extend_from_slice(CRLF);
    result
}

fn double_to_bytes(value: OrderedFloat<f64>) -> Vec<u8> {
    let repr = if value.is_infinite() {
        if value.is_sign_positive() {
            "inf"
        } else {
            "-inf"
        }
    } else if value.is_nan() {
        "nan"
    } else {
        return {
            let s = value.to_string();
            let mut result = Vec::with_capacity(s.len() + CRLF.len() + 1);
            result.extend_from_slice(b",");
            result.extend_from_slice(s.as_bytes());
            result.extend_from_slice(CRLF);
            result
        };
    };

    let mut result = Vec::with_capacity(repr.len() + CRLF.len() + 1);
    result.extend_from_slice(b",");
    result.extend_from_slice(repr.as_bytes());
    result.extend_from_slice(CRLF);
    result
}

fn big_number_to_bytes(value: &BigInt) -> Vec<u8> {
    let s = value.to_string();
    let mut result = Vec::with_capacity(s.len() + CRLF_LEN + 1);
    result.extend_from_slice(b"(");
    result.extend_from_slice(s.as_bytes());
    result.extend_from_slice(CRLF);
    result
}

fn bulk_error_to_bytes(payload: &str) -> Vec<u8> {
    let len_str = payload.len().to_string();

    let mut result = Vec::with_capacity(1 + len_str.len() + CRLF_LEN * 2 + payload.len());
    result.extend_from_slice(b"!");
    result.extend_from_slice(len_str.as_bytes());
    result.extend_from_slice(CRLF);
    result.extend_from_slice(payload.as_bytes());
    result.extend_from_slice(CRLF);
    result
}

fn map_to_bytes(map: &BTreeMap<Types, Types>) -> Vec<u8> {
    let mut result = Vec::with_capacity(32); // базовая длина + запас

    result.extend_from_slice(b"%");
    result.extend_from_slice(map.len().to_string().as_bytes());
    result.extend_from_slice(CRLF);

    for (key, value) in map {
        result.extend_from_slice(&key.to_bytes());
        result.extend_from_slice(&value.to_bytes());
    }

    result
}

fn set_to_bytes(set: &Vec<Types>) -> Vec<u8> {
    let mut result = Vec::with_capacity(32);

    result.extend_from_slice(b"~");
    result.extend_from_slice(set.len().to_string().as_bytes());
    result.extend_from_slice(CRLF);

    for elem in set {
        result.extend_from_slice(&elem.to_bytes());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigInt;
    use std::collections::BTreeMap;

    #[test]
    fn test_simple_string() {
        let val = Types::SimpleString("OK".to_string());
        assert_eq!(val.to_bytes(), b"+OK\r\n");
    }

    #[test]
    fn test_simple_error() {
        let val = Types::SimpleError("ERR something went wrong".to_string());
        assert_eq!(val.to_bytes(), b"-ERR something went wrong\r\n");
    }

    #[test]
    fn test_integer() {
        let val = Types::Integer(123);
        assert_eq!(val.to_bytes(), b":123\r\n");
    }

    #[test]
    fn test_bulk_string() {
        let val = Types::BulkString("foobar".to_string());
        assert_eq!(val.to_bytes(), b"$6\r\nfoobar\r\n");
    }

    #[test]
    fn test_array() {
        let val = Types::Array(vec![
            Types::SimpleString("foo".to_string()),
            Types::Integer(42),
        ]);
        assert_eq!(val.to_bytes(), b"*2\r\n+foo\r\n:42\r\n");
    }

    #[test]
    fn test_null() {
        let val = Types::Null;
        assert_eq!(val.to_bytes(), b"_\r\n");
    }

    #[test]
    fn test_boolean_true() {
        let val = Types::Boolean(true);
        assert_eq!(val.to_bytes(), b"#t\r\n");
    }

    #[test]
    fn test_boolean_false() {
        let val = Types::Boolean(false);
        assert_eq!(val.to_bytes(), b"#f\r\n");
    }

    #[test]
    fn test_double() {
        let val = Types::Double(OrderedFloat::from(3.14159));
        assert_eq!(val.to_bytes(), b",3.14159\r\n");
    }

    #[test]
    fn test_big_number() {
        let val =
            Types::BigNumber(BigInt::parse_bytes(b"123456789012345678901234567890", 10).unwrap());
        assert_eq!(val.to_bytes(), b"(123456789012345678901234567890\r\n");
    }

    #[test]
    fn test_bulk_error() {
        let val = Types::BulkError("ERR bulk error".to_string());
        assert_eq!(val.to_bytes(), b"!14\r\nERR bulk error\r\n");
    }

    #[test]
    fn test_map() {
        let mut map = BTreeMap::new();
        map.insert(Types::SimpleString("key".to_string()), Types::Integer(1));
        let val = Types::Map(map);
        assert_eq!(val.to_bytes(), b"%1\r\n+key\r\n:1\r\n");
    }

    #[test]
    fn test_set() {
        let val = Types::Set(vec![Types::Integer(1), Types::Integer(2)]);
        assert_eq!(val.to_bytes(), b"~2\r\n:1\r\n:2\r\n");
    }

    #[test]
    fn test_empty_array() {
        let val = Types::Array(vec![]);
        assert_eq!(val.to_bytes(), b"*0\r\n");
    }

    #[test]
    fn test_empty_map() {
        let val = Types::Map(BTreeMap::new());
        assert_eq!(val.to_bytes(), b"%0\r\n");
    }

    #[test]
    fn test_nested_array() {
        let val = Types::Array(vec![
            Types::Integer(1),
            Types::Array(vec![Types::Integer(2), Types::Integer(3)]),
        ]);
        assert_eq!(val.to_bytes(), b"*2\r\n:1\r\n*2\r\n:2\r\n:3\r\n");
    }

    #[test]
    fn test_null_in_array() {
        let val = Types::Array(vec![Types::Null, Types::SimpleString("value".to_string())]);
        assert_eq!(val.to_bytes(), b"*2\r\n_\r\n+value\r\n");
    }
}
