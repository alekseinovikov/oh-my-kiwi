use crate::core::error::ParseError;
use crate::core::BytesReader;
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

    pub(crate) async fn from_bytes<R: BytesReader + Send>(
        reader: &mut R,
    ) -> Result<Self, ParseError> {
        let line = reader.read_line().await?;
        let type_prefix = line[0];
        let rest_of_line = &line[1..];

        match type_prefix {
            b'+' => Ok(Types::SimpleString(String::from_utf8(
                rest_of_line.to_vec(),
            )?)),

            b'-' => Ok(Types::SimpleError(String::from_utf8(
                rest_of_line.to_vec(),
            )?)),

            b':' => {
                let s = String::from_utf8(rest_of_line.to_vec())?;
                Ok(Types::Integer(s.parse::<i64>()?))
            }

            b'_' => Ok(Types::Null),

            b'#' => {
                if rest_of_line.is_empty() {
                    return Err(ParseError::ExpectedBool);
                }
                match rest_of_line[0] {
                    b't' => Ok(Types::Boolean(true)),
                    b'f' => Ok(Types::Boolean(false)),
                    _ => Err(ParseError::ExpectedBool),
                }
            }

            b',' => {
                let s = String::from_utf8(rest_of_line.to_vec())?;
                let val = match s.as_str() {
                    "inf" => f64::INFINITY,
                    "-inf" => f64::NEG_INFINITY,
                    "nan" => f64::NAN,
                    _ => s.parse::<f64>()?,
                };
                Ok(Types::Double(OrderedFloat(val)))
            }

            b'(' => {
                let s = String::from_utf8(rest_of_line.to_vec())?;
                Ok(Types::BigNumber(
                    BigInt::parse_bytes(s.as_bytes(), 10)
                        .ok_or(ParseError::WrongBigNumberFormat)?,
                ))
            }

            b'$' => {
                let len_str = String::from_utf8(rest_of_line.to_vec())?;
                let len = len_str.parse::<isize>()?;

                if len == -1 {
                    return Ok(Types::Null); // Null Bulk String
                }

                let data_with_crlf = reader.read_bytes(len as usize + 2).await?;
                if &data_with_crlf[len as usize..] != CRLF {
                    return Err(ParseError::MissingSeparator);
                }
                Ok(Types::BulkString(String::from_utf8(
                    data_with_crlf[..len as usize].to_vec(),
                )?))
            }

            b'!' => {
                let len_str = String::from_utf8(rest_of_line.to_vec())?;
                let len = len_str.parse::<usize>()?;

                let data_with_crlf = reader.read_bytes(len + 2).await?;
                if &data_with_crlf[len..] != CRLF {
                    return Err(ParseError::MissingSeparator);
                }
                Ok(Types::BulkError(String::from_utf8(
                    data_with_crlf[..len].to_vec(),
                )?))
            }

            b'*' => {
                let len_str = String::from_utf8(rest_of_line.to_vec())?;
                let len = len_str.parse::<usize>()?;
                let mut arr = Vec::with_capacity(len);
                for _ in 0..len {
                    arr.push(Box::pin(Self::from_bytes(reader)).await?);
                }
                Ok(Types::Array(arr))
            }

            b'%' => {
                let len_str = String::from_utf8(rest_of_line.to_vec())?;
                let len = len_str.parse::<usize>()?;
                let mut map = BTreeMap::new();
                for _ in 0..len {
                    // ИЗМЕНЕНИЯ ЗДЕСЬ
                    let key = Box::pin(Self::from_bytes(reader)).await?;
                    let value = Box::pin(Self::from_bytes(reader)).await?;
                    map.insert(key, value);
                }
                Ok(Types::Map(map))
            }

            b'~' => {
                let len_str = String::from_utf8(rest_of_line.to_vec())?;
                let len = len_str.parse::<usize>()?;
                let mut set = Vec::with_capacity(len);
                for _ in 0..len {
                    // ИЗМЕНЕНИЕ ЗДЕСЬ
                    set.push(Box::pin(Self::from_bytes(reader)).await?);
                }
                Ok(Types::Set(set))
            }

            _ => {
                let type_symbol = String::from_utf8(vec![type_prefix])?;
                Err(ParseError::UnsupportedDataType(type_symbol))
            }
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
    use async_trait::async_trait;

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
        let val = Types::Double(OrderedFloat::from(23.4554));
        assert_eq!(val.to_bytes(), b",23.4554\r\n");
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

    struct MockReader<'a> {
        data: &'a [u8],
        pos: usize,
    }

    impl<'a> MockReader<'a> {
        fn new(data: &'a [u8]) -> Self {
            Self { data, pos: 0 }
        }
    }

    #[async_trait]
    impl<'a> BytesReader for MockReader<'a> {
        async fn read_line(&mut self) -> Result<Vec<u8>, ParseError> {
            if self.pos >= self.data.len() {
                return Err(ParseError::ConnectionClosed);
            }

            if let Some(i) = self.data[self.pos..].windows(2).position(|w| w == b"\r\n") {
                let line_end = self.pos + i;
                let line = self.data[self.pos..line_end].to_vec();
                self.pos = line_end + 2;
                Ok(line)
            } else {
                Err(ParseError::ConnectionClosed)
            }
        }

        async fn read_bytes(&mut self, n: usize) -> Result<Vec<u8>, ParseError> {
            let bytes_end = self.pos + n;
            if bytes_end > self.data.len() {
                return Err(ParseError::ConnectionClosed);
            }
            let bytes = self.data[self.pos..bytes_end].to_vec();
            self.pos = bytes_end;
            Ok(bytes)
        }
    }

    #[tokio::test]
    async fn test_parse_simple_string() {
        let input = b"+OK\r\n";
        let mut reader = MockReader::new(input);
        let result = Types::from_bytes(&mut reader).await.unwrap();
        assert_eq!(result, Types::SimpleString("OK".to_string()));
    }

    #[tokio::test]
    async fn test_parse_simple_error() {
        let input = b"-Error message\r\n";
        let mut reader = MockReader::new(input);
        let result = Types::from_bytes(&mut reader).await.unwrap();
        assert_eq!(result, Types::SimpleError("Error message".to_string()));
    }

    #[tokio::test]
    async fn test_parse_integer() {
        let input = b":12345\r\n";
        let mut reader = MockReader::new(input);
        let result = Types::from_bytes(&mut reader).await.unwrap();
        assert_eq!(result, Types::Integer(12345));
    }

    #[tokio::test]
    async fn test_parse_bulk_string() {
        let input = b"$6\r\nfoobar\r\n";
        let mut reader = MockReader::new(input);
        let result = Types::from_bytes(&mut reader).await.unwrap();
        assert_eq!(result, Types::BulkString("foobar".to_string()));
    }

    #[tokio::test]
    async fn test_parse_empty_bulk_string() {
        let input = b"$0\r\n\r\n";
        let mut reader = MockReader::new(input);
        let result = Types::from_bytes(&mut reader).await.unwrap();
        assert_eq!(result, Types::BulkString("".to_string()));
    }

    #[tokio::test]
    async fn test_parse_null_bulk_string() {
        let input = b"$-1\r\n";
        let mut reader = MockReader::new(input);
        let result = Types::from_bytes(&mut reader).await.unwrap();
        assert_eq!(result, Types::Null);
    }

    #[tokio::test]
    async fn test_parse_null() {
        let input = b"_\r\n";
        let mut reader = MockReader::new(input);
        let result = Types::from_bytes(&mut reader).await.unwrap();
        assert_eq!(result, Types::Null);
    }

    #[tokio::test]
    async fn test_parse_boolean() {
        let input_true = b"#t\r\n";
        let mut reader_true = MockReader::new(input_true);
        let result_true = Types::from_bytes(&mut reader_true).await.unwrap();
        assert_eq!(result_true, Types::Boolean(true));

        let input_false = b"#f\r\n";
        let mut reader_false = MockReader::new(input_false);
        let result_false = Types::from_bytes(&mut reader_false).await.unwrap();
        assert_eq!(result_false, Types::Boolean(false));
    }

    #[tokio::test]
    async fn test_parse_double() {
        let input = b",1.234\r\n";
        let mut reader = MockReader::new(input);
        let result = Types::from_bytes(&mut reader).await.unwrap();
        assert_eq!(result, Types::Double(OrderedFloat(1.234)));
    }

    #[tokio::test]
    async fn test_parse_double_inf() {
        let input = b",inf\r\n";
        let mut reader = MockReader::new(input);
        let result = Types::from_bytes(&mut reader).await.unwrap();
        assert_eq!(result, Types::Double(OrderedFloat(f64::INFINITY)));
    }

    #[tokio::test]
    async fn test_parse_big_number() {
        let input = b"(12345678901234567890\r\n";
        let mut reader = MockReader::new(input);
        let result = Types::from_bytes(&mut reader).await.unwrap();
        let expected = BigInt::parse_bytes(b"12345678901234567890", 10).unwrap();
        assert_eq!(result, Types::BigNumber(expected));
    }

    #[tokio::test]
    async fn test_parse_bulk_error() {
        let input = b"!13\r\nError message\r\n";
        let mut reader = MockReader::new(input);
        let result = Types::from_bytes(&mut reader).await.unwrap();
        assert_eq!(result, Types::BulkError("Error message".to_string()));
    }

    #[tokio::test]
    async fn test_parse_array() {
        let input = b"*2\r\n$3\r\nfoo\r\n:42\r\n";
        let mut reader = MockReader::new(input);
        let result = Types::from_bytes(&mut reader).await.unwrap();
        let expected = Types::Array(vec![
            Types::BulkString("foo".to_string()),
            Types::Integer(42),
        ]);
        assert_eq!(result, expected);
    }

    #[tokio::test]
    async fn test_parse_empty_array() {
        let input = b"*0\r\n";
        let mut reader = MockReader::new(input);
        let result = Types::from_bytes(&mut reader).await.unwrap();
        assert_eq!(result, Types::Array(vec![]));
    }

    #[tokio::test]
    async fn test_parse_nested_array() {
        let input = b"*2\r\n:1\r\n*2\r\n+two\r\n+three\r\n";
        let mut reader = MockReader::new(input);
        let result = Types::from_bytes(&mut reader).await.unwrap();
        let expected = Types::Array(vec![
            Types::Integer(1),
            Types::Array(vec![
                Types::SimpleString("two".to_string()),
                Types::SimpleString("three".to_string()),
            ]),
        ]);
        assert_eq!(result, expected);
    }

    #[tokio::test]
    async fn test_parse_map() {
        let input = b"%2\r\n+key1\r\n:1\r\n+key2\r\n:2\r\n";
        let mut reader = MockReader::new(input);
        let result = Types::from_bytes(&mut reader).await.unwrap();

        let mut expected_map = BTreeMap::new();
        expected_map.insert(Types::SimpleString("key1".to_string()), Types::Integer(1));
        expected_map.insert(Types::SimpleString("key2".to_string()), Types::Integer(2));

        assert_eq!(result, Types::Map(expected_map));
    }

    #[tokio::test]
    async fn test_parse_empty_map() {
        let input = b"%0\r\n";
        let mut reader = MockReader::new(input);
        let result = Types::from_bytes(&mut reader).await.unwrap();
        assert_eq!(result, Types::Map(BTreeMap::new()));
    }

    #[tokio::test]
    async fn test_parse_set() {
        let input = b"~3\r\n+one\r\n:2\r\n#t\r\n";
        let mut reader = MockReader::new(input);
        let result = Types::from_bytes(&mut reader).await.unwrap();
        let expected = Types::Set(vec![
            Types::SimpleString("one".to_string()),
            Types::Integer(2),
            Types::Boolean(true),
        ]);
        assert_eq!(result, expected);
    }
}
