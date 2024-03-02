use crate::error::{Error, ExecutableError, Result};

pub fn get_u8(input: &[u8], offset: &mut usize) -> Result<u8> {
    let bytes = get_bytes(input, offset, 1)?;
    let result = u8::from_be_bytes(
        bytes[0..1]
            .try_into()
            .map_err(|_| Error::Executable(ExecutableError::FailedDeserialize))?,
    );
    Ok(result)
}

pub fn get_u16(input: &[u8], offset: &mut usize) -> Result<u16> {
    let bytes = get_bytes(input, offset, 2)?;
    let result = u16::from_be_bytes(
        bytes[0..2]
            .try_into()
            .map_err(|_| Error::Executable(ExecutableError::FailedDeserialize))?,
    );
    Ok(result)
}

pub fn get_u32(input: &[u8], offset: &mut usize) -> Result<u32> {
    let bytes = get_bytes(input, offset, 4)?;
    let result = u32::from_be_bytes(
        bytes[0..4]
            .try_into()
            .map_err(|_| Error::Executable(ExecutableError::FailedDeserialize))?,
    );
    Ok(result)
}

pub fn get_u64(input: &[u8], offset: &mut usize) -> Result<u64> {
    let bytes = get_bytes(input, offset, 8)?;
    let result = u64::from_be_bytes(
        bytes[0..8]
            .try_into()
            .map_err(|_| Error::Executable(ExecutableError::FailedDeserialize))?,
    );
    Ok(result)
}

pub fn get_i64(input: &[u8], offset: &mut usize) -> Result<i64> {
    let bytes = get_bytes(input, offset, 8)?;
    let result = i64::from_be_bytes(
        bytes[0..8]
            .try_into()
            .map_err(|_| Error::Executable(ExecutableError::FailedDeserialize))?,
    );
    Ok(result)
}

pub fn get_bytes(input: &[u8], offset: &mut usize, length: usize) -> Result<Vec<u8>> {
    let offset_input = *offset;
    match input.get(offset_input..offset_input + length) {
        Some(bytes) => {
            *offset += length;
            Ok(bytes.to_vec())
        }
        None => Err(Error::Executable(ExecutableError::FailedDeserialize)),
    }
}
