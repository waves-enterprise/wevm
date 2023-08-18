use crate::{exec::ExecutableError, Error, Result};

pub enum DataEntry<'a> {
    Integer(i64),
    Boolean(i32),
    Binary(&'a [u8]),
    String(&'a [u8]),
}

impl<'a> DataEntry<'a> {
    pub fn serialize(&self, key: &[u8]) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];

        result.extend_from_slice(&(key.len() as u16).to_be_bytes());
        result.extend_from_slice(key);

        match self {
            DataEntry::Integer(value) => {
                result.push(0u8);
                result.extend_from_slice(&value.to_be_bytes());
            }
            DataEntry::Boolean(value) => {
                result.push(1u8);
                result.push(*value as u8);
            }
            DataEntry::Binary(value) => {
                result.push(2u8);
                result.extend_from_slice(&(value.len() as u32).to_be_bytes());
                result.extend_from_slice(value);
            }
            DataEntry::String(value) => {
                result.push(3u8);
                result.extend_from_slice(&(value.len() as u32).to_be_bytes());
                result.extend_from_slice(value);
            }
        }

        result
    }
}

pub fn parse(input: &[u8], memory: &mut [u8], offset_memory: &mut usize) -> Result<Vec<String>> {
    let mut offset_input: usize = 0;

    let mut args: Vec<String> = vec![];

    if input.is_empty() {
        return Ok(args);
    }

    let mut count = get_byte(input, &mut offset_input)?;
    while count > 0 {
        skip_key(input, &mut offset_input)?;
        get_value(input, &mut offset_input, memory, offset_memory, &mut args)?;
        count -= 1;
    }

    Ok(args)
}

fn get_byte(input: &[u8], offset: &mut usize) -> Result<u8> {
    match input.get(*offset) {
        Some(byte) => {
            *offset += 1;
            Ok(*byte)
        }
        None => Err(Error::Executable(ExecutableError::FailedParseDataEntry)),
    }
}

fn get_bytes(
    input: &[u8],
    offset_input: &mut usize,
    length: usize,
    value: &mut [u8],
    offset_value: &mut usize,
) -> Result<()> {
    let offset_i = *offset_input;
    match input.get(offset_i..offset_i + length) {
        Some(bytes) => {
            let offset_v = *offset_value;
            value[offset_v..offset_v + length].copy_from_slice(&bytes[..length]);
            *offset_input += length;
            *offset_value += length;
        }
        None => return Err(Error::Executable(ExecutableError::FailedParseDataEntry)),
    }

    Ok(())
}

fn get_u16(input: &[u8], offset: &mut usize) -> Result<u16> {
    let mut temp = [0u8; 2];
    let mut temp_offset = 0;
    get_bytes(input, offset, 2, &mut temp, &mut temp_offset)?;
    Ok(u16::from_be_bytes(temp))
}

fn get_u64(input: &[u8], offset: &mut usize) -> Result<u64> {
    let mut temp = [0u8; 8];
    let mut temp_offset = 0;
    get_bytes(input, offset, 8, &mut temp, &mut temp_offset)?;
    Ok(u64::from_be_bytes(temp))
}

fn skip_key(input: &[u8], offset: &mut usize) -> Result<()> {
    let length = get_u16(input, offset)?;
    *offset += length as usize;
    Ok(())
}

fn get_value(
    input: &[u8],
    offset_input: &mut usize,
    memory: &mut [u8],
    offset_memory: &mut usize,
    args: &mut Vec<String>,
) -> Result<()> {
    let byte = get_byte(input, offset_input)?;

    match byte {
        0u8 => {
            // Integer
            let int = get_u64(input, offset_input)?;
            args.push(format!("{}", int));
        }
        1u8 => {
            // Boolean
            let byte = get_byte(input, offset_input)?;
            args.push(format!("{}", byte));
        }
        2u8 => {
            // Binary
            let length = get_u16(input, offset_input)?;
            args.push(format!("{}", *offset_memory));
            args.push(format!("{}", length));
            get_bytes(input, offset_input, length as usize, memory, offset_memory)?;
        }
        3u8 => {
            // String
            let length = get_u16(input, offset_input)?;
            args.push(format!("{}", *offset_memory));
            args.push(format!("{}", length));
            get_bytes(input, offset_input, length as usize, memory, offset_memory)?;
        }
        _ => return Err(Error::Executable(ExecutableError::FailedParseDataEntry)),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test() {
        let input = [
            4, 0, 8, 116, 101, 115, 116, 95, 107, 101, 121, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 8, 116,
            101, 115, 116, 95, 107, 101, 121, 1, 1, 0, 8, 116, 101, 115, 116, 95, 107, 101, 121, 2,
            0, 10, 116, 101, 115, 116, 95, 118, 97, 108, 117, 101, 0, 8, 116, 101, 115, 116, 95,
            107, 101, 121, 3, 0, 10, 116, 101, 115, 116, 95, 118, 97, 108, 117, 101,
        ];

        let mut memory = [0u8; 1000];
        let mut offset_memory = 100;

        let result = parse(&input, &mut memory, &mut offset_memory).expect("Error parse DataEntry");

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
