use crate::error::{Error, Result, RuntimeError};

pub enum Type {
    Account = 0,
    Contract = 1,
}

impl TryFrom<u32> for Type {
    type Error = Error;

    fn try_from(type_: u32) -> Result<Self, Self::Error> {
        match type_ {
            0 => Ok(Type::Account),
            1 => Ok(Type::Contract),
            _ => Err(Error::Runtime(RuntimeError::AssetHolderTypeNotFound)),
        }
    }
}

pub enum AddressVersion {
    Address = 1,
    Alias = 2,
}

impl TryFrom<u32> for AddressVersion {
    type Error = Error;

    fn try_from(type_: u32) -> Result<Self, Self::Error> {
        match type_ {
            1 => Ok(AddressVersion::Address),
            2 => Ok(AddressVersion::Alias),
            _ => Err(Error::Runtime(RuntimeError::AddressVersionNotFound)),
        }
    }
}

pub struct AssetHolder(Vec<u8>);

impl AssetHolder {
    pub fn from_bytes(type_: Type, version: AddressVersion, chain_id: u8, bytes: Vec<u8>) -> Self {
        let mut result: Vec<u8> = vec![];

        match type_ {
            Type::Account => match version {
                AddressVersion::Address => result.push(0),
                AddressVersion::Alias => result.extend(vec![0, 2, chain_id]),
            },
            Type::Contract => result.push(1),
        }

        result.extend(bytes);
        AssetHolder(result)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }
}
