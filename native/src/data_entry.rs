use crate::{exec::ExecutableError, Error, Result};

#[derive(Debug, Eq, PartialEq)]
pub enum DataEntry {
    Integer(i64),
    Boolean(i32),
    Binary(Vec<u8>),
    String(Vec<u8>),
}

impl DataEntry {
    pub fn serialize(&self, key: &[u8]) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];

        result.extend_from_slice(&(key.len() as u16).to_be_bytes());
        result.extend_from_slice(key);

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

    pub fn deserialize_storage(input: &[u8]) -> Result<Self> {
        let mut offset_input: usize = 0;

        Self::skip_key(input, &mut offset_input)?;
        Self::get_value(input, &mut offset_input)
    }

    pub fn deserialize_args(
        input: &[u8],
        output: &mut [u8],
        offset_output: &mut usize,
    ) -> Result<Vec<String>> {
        let mut offset_input: usize = 0;

        let mut args: Vec<String> = vec![];

        if input.is_empty() {
            return Ok(args);
        }

        let mut count = Self::get_u8(input, &mut offset_input)?;
        while count > 0 {
            Self::skip_key(input, &mut offset_input)?;
            match Self::get_value(input, &mut offset_input)? {
                Self::Integer(value) => args.push(format!("{}", value)),
                Self::Boolean(value) => args.push(format!("{}", value)),
                Self::Binary(value) => {
                    let length = value.len();
                    let offset_o = *offset_output;
                    output[offset_o..offset_o + length].copy_from_slice(value.as_slice());
                    args.push(format!("{}", *offset_output));
                    args.push(format!("{}", length));
                }
                Self::String(value) => {
                    let length = value.len();
                    let offset_o = *offset_output;
                    output[offset_o..offset_o + length].copy_from_slice(value.as_slice());
                    args.push(format!("{}", *offset_output));
                    args.push(format!("{}", length));
                }
            }
            count -= 1;
        }

        Ok(args)
    }

    fn skip_key(input: &[u8], offset: &mut usize) -> Result<()> {
        let length = Self::get_u16(input, offset)?;
        *offset += length as usize;
        Ok(())
    }

    fn get_value(input: &[u8], offset: &mut usize) -> Result<Self> {
        let byte = Self::get_u8(input, offset)?;

        match byte {
            0u8 => {
                let integer = Self::get_u64(input, offset)?;
                Ok(Self::Integer(integer as i64))
            }
            1u8 => {
                let boolean = Self::get_u8(input, offset)?;
                Ok(Self::Boolean(boolean as i32))
            }
            2u8 => {
                let length = Self::get_u16(input, offset)?;
                let binary = Self::get_bytes(input, offset, length as usize)?;
                Ok(Self::Binary(binary))
            }
            3u8 => {
                let length = Self::get_u16(input, offset)?;
                let string = Self::get_bytes(input, offset, length as usize)?;
                Ok(Self::String(string))
            }
            _ => {
                return Err(Error::Executable(
                    ExecutableError::FailedDeserializeDataEntry,
                ))
            }
        }
    }

    fn get_u8(input: &[u8], offset: &mut usize) -> Result<u8> {
        let bytes = Self::get_bytes(input, offset, 1)?;
        let result = u8::from_be_bytes(
            bytes[0..1]
                .try_into()
                .map_err(|_| Error::Executable(ExecutableError::FailedDeserializeDataEntry))?,
        );
        Ok(result)
    }

    fn get_u16(input: &[u8], offset: &mut usize) -> Result<u16> {
        let bytes = Self::get_bytes(input, offset, 2)?;
        let result = u16::from_be_bytes(
            bytes[0..2]
                .try_into()
                .map_err(|_| Error::Executable(ExecutableError::FailedDeserializeDataEntry))?,
        );
        Ok(result)
    }

    fn get_u64(input: &[u8], offset: &mut usize) -> Result<u64> {
        let bytes = Self::get_bytes(input, offset, 8)?;
        let result = u64::from_be_bytes(
            bytes[0..8]
                .try_into()
                .map_err(|_| Error::Executable(ExecutableError::FailedDeserializeDataEntry))?,
        );
        Ok(result)
    }

    fn get_bytes(input: &[u8], offset: &mut usize, length: usize) -> Result<Vec<u8>> {
        let offset_input = *offset;
        match input.get(offset_input..offset_input + length) {
            Some(bytes) => {
                *offset += length;
                Ok(bytes.to_vec())
            }
            None => {
                return Err(Error::Executable(
                    ExecutableError::FailedDeserializeDataEntry,
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_deserialize_storage() {
        let input = [
            0, 8, 116, 101, 115, 116, 95, 107, 101, 121, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ];
        let result = DataEntry::deserialize_storage(&input).expect("Error deserialize DataEntry");
        assert_eq!(result, DataEntry::Integer(1));

        let input = [0, 8, 116, 101, 115, 116, 95, 107, 101, 121, 1, 1];
        let result = DataEntry::deserialize_storage(&input).expect("Error deserialize DataEntry");
        assert_eq!(result, DataEntry::Boolean(1));

        let vec: Vec<u8> = vec![116, 101, 115, 116, 95, 118, 97, 108, 117, 101];

        let input = [
            0, 8, 116, 101, 115, 116, 95, 107, 101, 121, 2, 0, 10, 116, 101, 115, 116, 95, 118, 97,
            108, 117, 101,
        ];
        let result = DataEntry::deserialize_storage(&input).expect("Error deserialize DataEntry");
        assert_eq!(result, DataEntry::Binary(vec.clone()));

        let input = [
            0, 8, 116, 101, 115, 116, 95, 107, 101, 121, 3, 0, 10, 116, 101, 115, 116, 95, 118, 97,
            108, 117, 101,
        ];
        let result = DataEntry::deserialize_storage(&input).expect("Error deserialize DataEntry");
        assert_eq!(result, DataEntry::String(vec.clone()));
    }

    #[test]
    fn test_deserialize_args() {
        let input = [
            4, 0, 8, 116, 101, 115, 116, 95, 107, 101, 121, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 8, 116,
            101, 115, 116, 95, 107, 101, 121, 1, 1, 0, 8, 116, 101, 115, 116, 95, 107, 101, 121, 2,
            0, 10, 116, 101, 115, 116, 95, 118, 97, 108, 117, 101, 0, 8, 116, 101, 115, 116, 95,
            107, 101, 121, 3, 0, 10, 116, 101, 115, 116, 95, 118, 97, 108, 117, 101,
        ];

        let mut memory = [0u8; 1000];
        let mut offset_memory = 100;

        let result = DataEntry::deserialize_args(&input, &mut memory, &mut offset_memory)
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
