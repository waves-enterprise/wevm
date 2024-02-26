use crate::{
    error::{Error, ExecutableError, Result, RuntimeError},
    runtime::utils,
};

#[derive(Debug, Eq, PartialEq)]
pub enum DataEntry {
    Integer(i64),
    Boolean(i32),
    Binary(Vec<u8>),
    String(Vec<u8>),
}

impl DataEntry {
    pub fn serialize(&self, key: Option<&[u8]>) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];

        match key {
            Some(bytes) => {
                result.extend_from_slice(&(bytes.len() as u16).to_be_bytes());
                result.extend_from_slice(bytes);
            }
            None => result.extend_from_slice(&0u16.to_be_bytes()),
        }

        match self {
            Self::Integer(value) => {
                result.push(0u8);
                result.extend_from_slice(&value.to_be_bytes());
            }
            Self::Boolean(value) => {
                result.push(1u8);
                result.push(*value as u8);
            }
            Self::Binary(value) => {
                result.push(2u8);
                result.extend_from_slice(&(value.len() as u32).to_be_bytes());
                result.extend_from_slice(value);
            }
            Self::String(value) => {
                result.push(3u8);
                result.extend_from_slice(&(value.len() as u32).to_be_bytes());
                result.extend_from_slice(value);
            }
        }

        result
    }

    pub fn serialize_slice(data_entry: &[DataEntry]) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];

        if !data_entry.is_empty() {
            result.extend_from_slice(&(data_entry.len() as u16).to_be_bytes());

            for item in data_entry {
                result.extend(item.serialize(None));
            }
        }

        result
    }

    pub fn deserialize(input: &[u8]) -> Result<Self> {
        let mut offset_input: usize = 0;

        Self::skip_key(input, &mut offset_input)?;
        Self::get_value(input, &mut offset_input)
    }

    pub fn deserialize_with_key(input: &[u8]) -> Result<(String, Self)> {
        let mut offset_input: usize = 0;

        let key = Self::get_key(input, &mut offset_input)?;
        let value = Self::get_value(input, &mut offset_input)?;
        Ok((key, value))
    }

    pub fn deserialize_params(
        input: &[u8],
        output: &mut [u8],
        offset_output: &mut usize,
    ) -> Result<Vec<String>> {
        let mut offset_input: usize = 0;

        let mut params: Vec<String> = vec![];

        if input.is_empty() {
            return Ok(params);
        }

        let mut count = utils::get_u16(input, &mut offset_input)?;
        while count > 0 {
            Self::skip_key(input, &mut offset_input)?;
            match Self::get_value(input, &mut offset_input)? {
                Self::Integer(value) => params.push(format!("{}", value)),
                Self::Boolean(value) => params.push(format!("{}", value)),
                Self::Binary(value) => {
                    let length = value.len();
                    let offset_o = *offset_output;
                    output[offset_o..offset_o + length].copy_from_slice(value.as_slice());
                    params.push(format!("{}", *offset_output));
                    params.push(format!("{}", length));
                    *offset_output += length;
                }
                Self::String(value) => {
                    let length = value.len();
                    let offset_o = *offset_output;
                    output[offset_o..offset_o + length].copy_from_slice(value.as_slice());
                    params.push(format!("{}", *offset_output));
                    params.push(format!("{}", length));
                    *offset_output += length;
                }
            }
            count -= 1;
        }

        Ok(params)
    }

    fn get_key(input: &[u8], offset: &mut usize) -> Result<String> {
        let length = utils::get_u16(input, offset)?;
        let key = utils::get_bytes(input, offset, length as usize)?;
        String::from_utf8(key).map_err(|_| Error::Runtime(RuntimeError::Utf8Error))
    }

    fn skip_key(input: &[u8], offset: &mut usize) -> Result<()> {
        let length = utils::get_u16(input, offset)?;
        *offset += length as usize;
        Ok(())
    }

    fn get_value(input: &[u8], offset: &mut usize) -> Result<Self> {
        let byte = utils::get_u8(input, offset)?;

        match byte {
            0u8 => {
                let integer = utils::get_u64(input, offset)?;
                Ok(Self::Integer(integer as i64))
            }
            1u8 => {
                let boolean = utils::get_u8(input, offset)?;
                Ok(Self::Boolean(boolean as i32))
            }
            2u8 => {
                let length = utils::get_u32(input, offset)?;
                let binary = utils::get_bytes(input, offset, length as usize)?;
                Ok(Self::Binary(binary))
            }
            3u8 => {
                let length = utils::get_u32(input, offset)?;
                let string = utils::get_bytes(input, offset, length as usize)?;
                Ok(Self::String(string))
            }
            _ => Err(Error::Executable(ExecutableError::FailedDeserialize)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_deserialize() {
        let input = [
            0, 8, 116, 101, 115, 116, 95, 107, 101, 121, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ];
        let result = DataEntry::deserialize(&input).expect("Error deserialize DataEntry");
        assert_eq!(result, DataEntry::Integer(1));

        let input = [0, 8, 116, 101, 115, 116, 95, 107, 101, 121, 1, 1];
        let result = DataEntry::deserialize(&input).expect("Error deserialize DataEntry");
        assert_eq!(result, DataEntry::Boolean(1));

        let vec: Vec<u8> = vec![116, 101, 115, 116, 95, 118, 97, 108, 117, 101];

        let input = [
            0, 8, 116, 101, 115, 116, 95, 107, 101, 121, 2, 0, 0, 0, 10, 116, 101, 115, 116, 95,
            118, 97, 108, 117, 101,
        ];
        let result = DataEntry::deserialize(&input).expect("Error deserialize DataEntry");
        assert_eq!(result, DataEntry::Binary(vec.clone()));

        let input = [
            0, 8, 116, 101, 115, 116, 95, 107, 101, 121, 3, 0, 0, 0, 10, 116, 101, 115, 116, 95,
            118, 97, 108, 117, 101,
        ];
        let result = DataEntry::deserialize(&input).expect("Error deserialize DataEntry");
        assert_eq!(result, DataEntry::String(vec.clone()));
    }

    #[test]
    fn test_deserialize_with_key() {
        let input = [
            0, 8, 116, 101, 115, 116, 95, 107, 101, 121, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ];
        let (key, value) =
            DataEntry::deserialize_with_key(&input).expect("Error deserialize DataEntry");

        assert_eq!(key, "test_key".to_string());
        assert_eq!(value, DataEntry::Integer(1));
    }

    #[test]
    fn test_deserialize_params() {
        let input = [
            0, 4, 0, 8, 116, 101, 115, 116, 95, 107, 101, 121, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 8,
            116, 101, 115, 116, 95, 107, 101, 121, 1, 1, 0, 8, 116, 101, 115, 116, 95, 107, 101,
            121, 2, 0, 0, 0, 10, 116, 101, 115, 116, 95, 118, 97, 108, 117, 101, 0, 8, 116, 101,
            115, 116, 95, 107, 101, 121, 3, 0, 0, 0, 10, 116, 101, 115, 116, 95, 118, 97, 108, 117,
            101,
        ];

        let mut memory = [0u8; 1000];
        let mut offset_memory = 100;

        let result = DataEntry::deserialize_params(&input, &mut memory, &mut offset_memory)
            .expect("Error deserialize DataEntry");

        assert_eq!(result.len(), 6);
        assert_eq!(result[0], "1");
        assert_eq!(result[1], "1");

        let data = [116, 101, 115, 116, 95, 118, 97, 108, 117, 101];

        let offset = usize::from_str(&result[2]).expect("Failed usize from_str");
        let length = usize::from_str(&result[3]).expect("Failed usize from_str");
        assert_eq!(memory[offset..offset + length], data);

        let offset = usize::from_str(&result[4]).expect("Failed usize from_str");
        let length = usize::from_str(&result[5]).expect("Failed usize from_str");
        assert_eq!(memory[offset..offset + length], data);
    }
}
