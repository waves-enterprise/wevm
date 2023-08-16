use crate::{stack::Stack, Error, Result};
use jni::objects::{JByteArray, JObject, JValue};

/// A primitive java type.
/// L - Object
/// Z - Boolean
/// I - Integer
/// J - Long
/// V - Void

#[derive(Copy, Clone, Debug)]
pub enum JvmError {
    /// Failed attaches the current thread to the Java VM
    AttachCurrentThread = 200,
    /// Failed JVM method call
    MethodCall = 201,
    /// Failed byte array conversion
    ByteArrayConversion = 202,
    /// Failed receiving JavaVM interface
    GetJavaVM = 203,
    /// Error callback new_global_ref
    NewGlobalRef = 204,
    /// Couldn't create java byte array
    NewByteArray = 205,
    /// Couldn't create java string
    NewString = 206,
    /// Failed to receive object
    ReceiveObject = 207,
    /// Failed to receive long
    ReceiveLong = 208,
    /// Failed to receive int
    ReceiveInt = 209,
}

pub trait Jvm {
    fn get_bytecode(&self, contract_id: &[u8]) -> Result<Vec<u8>>;
    fn get_storage(&self, contract_id: &[u8], key: &[u8]) -> Result<Vec<u8>>;
    fn set_storage(&self, key: &[u8], data_type: &str, value: &[u8]) -> Result<()>;
    fn get_balance(&self, asset_id: &[u8], address: &[u8]) -> Result<i64>;
    fn transfer(&self, asset_id: &[u8], recipient: &[u8], amount: i64) -> Result<()>;
    fn issue(
        &self,
        name: &[u8],
        description: &[u8],
        quantity: i64,
        decimals: i32,
        is_reissuable: bool,
    ) -> Result<Vec<u8>>;
    fn burn(&self, asset_id: &[u8], amount: i64) -> Result<()>;
    fn reissue(&self, asset_id: &[u8], amount: i64, is_reissuable: bool) -> Result<()>;
    fn lease(&self, recipient: &[u8], amount: i64) -> Result<Vec<u8>>;
    fn cancel_lease(&self, lease_id: &[u8]) -> Result<()>;
    fn get_block_timestamp(&self) -> Result<i64>;
    fn get_block_height(&self) -> Result<i64>;
    fn get_tx_sender(&self) -> Result<Vec<u8>>;
    fn get_tx_payments(&self) -> Result<i32>;
    fn get_tx_payment_asset_id(&self, number: i32) -> Result<Vec<u8>>;
    fn get_tx_payment_amount(&self, number: i32) -> Result<i64>;
}

// Implementing the JVM call
impl Jvm for Stack {
    fn get_bytecode(&self, contract_id: &[u8]) -> Result<Vec<u8>> {
        let mut env = self
            .jvm
            .attach_current_thread()
            .map_err(|_| Error::Jvm(JvmError::AttachCurrentThread))?;

        let contract_id = env
            .byte_array_from_slice(contract_id)
            .map_err(|_| Error::Jvm(JvmError::NewByteArray))?;

        let result = env
            .call_method(
                self.jvm_callback.clone(),
                "getBytecode",
                "([B)[B",
                &[JValue::Object(&contract_id.into())],
            )
            .map_err(|_| Error::Jvm(JvmError::MethodCall))?
            .l()
            .map_err(|_| Error::Jvm(JvmError::ReceiveObject))?;

        let bytes = env
            .convert_byte_array(<JObject<'_> as Into<JByteArray>>::into(result))
            .map_err(|_| Error::Jvm(JvmError::ByteArrayConversion))?;

        Ok(bytes.to_vec())
    }

    fn get_storage(&self, contract_id: &[u8], key: &[u8]) -> Result<Vec<u8>> {
        let mut env = self
            .jvm
            .attach_current_thread()
            .map_err(|_| Error::Jvm(JvmError::AttachCurrentThread))?;

        let contract_id = env
            .byte_array_from_slice(contract_id)
            .map_err(|_| Error::Jvm(JvmError::NewByteArray))?;

        let key = env
            .byte_array_from_slice(key)
            .map_err(|_| Error::Jvm(JvmError::NewByteArray))?;

        let result = env
            .call_method(
                self.jvm_callback.clone(),
                "getStorage",
                "([B[B)[B",
                &[
                    JValue::Object(&contract_id.into()),
                    JValue::Object(&key.into()),
                ],
            )
            .map_err(|_| Error::Jvm(JvmError::MethodCall))?
            .l()
            .map_err(|_| Error::Jvm(JvmError::ReceiveObject))?;

        let bytes = env
            .convert_byte_array(<JObject<'_> as Into<JByteArray>>::into(result))
            .map_err(|_| Error::Jvm(JvmError::ByteArrayConversion))?;

        Ok(bytes.to_vec())
    }

    fn set_storage(&self, key: &[u8], data_type: &str, value: &[u8]) -> Result<()> {
        let mut env = self
            .jvm
            .attach_current_thread()
            .map_err(|_| Error::Jvm(JvmError::AttachCurrentThread))?;

        let key = env
            .byte_array_from_slice(key)
            .map_err(|_| Error::Jvm(JvmError::NewByteArray))?;

        let data_type = env
            .new_string(data_type)
            .map_err(|_| Error::Jvm(JvmError::NewString))?;

        let value = env
            .byte_array_from_slice(value)
            .map_err(|_| Error::Jvm(JvmError::NewByteArray))?;

        env.call_method(
            self.jvm_callback.clone(),
            "setStorage",
            "([BLjava/lang/String;[B)V",
            &[
                JValue::Object(&key.into()),
                JValue::Object(&data_type.into()),
                JValue::Object(&value.into()),
            ],
        )
        .map_err(|_| Error::Jvm(JvmError::MethodCall))?;

        Ok(())
    }

    fn get_balance(&self, asset_id: &[u8], address: &[u8]) -> Result<i64> {
        let mut env = self
            .jvm
            .attach_current_thread()
            .map_err(|_| Error::Jvm(JvmError::AttachCurrentThread))?;

        let asset_id = env
            .byte_array_from_slice(asset_id)
            .map_err(|_| Error::Jvm(JvmError::NewByteArray))?;

        let address = env
            .byte_array_from_slice(address)
            .map_err(|_| Error::Jvm(JvmError::NewByteArray))?;

        let result = env
            .call_method(
                self.jvm_callback.clone(),
                "getBalance",
                "([B[B)J",
                &[
                    JValue::Object(&asset_id.into()),
                    JValue::Object(&address.into()),
                ],
            )
            .map_err(|_| Error::Jvm(JvmError::MethodCall))?
            .j()
            .map_err(|_| Error::Jvm(JvmError::ReceiveLong))?;

        Ok(result)
    }

    fn transfer(&self, asset_id: &[u8], recipient: &[u8], amount: i64) -> Result<()> {
        let mut env = self
            .jvm
            .attach_current_thread()
            .map_err(|_| Error::Jvm(JvmError::AttachCurrentThread))?;

        let asset_id = env
            .byte_array_from_slice(asset_id)
            .map_err(|_| Error::Jvm(JvmError::NewByteArray))?;

        let recipient = env
            .byte_array_from_slice(recipient)
            .map_err(|_| Error::Jvm(JvmError::NewByteArray))?;

        env.call_method(
            self.jvm_callback.clone(),
            "transfer",
            "([B[BJ)V",
            &[
                JValue::Object(&asset_id.into()),
                JValue::Object(&recipient.into()),
                amount.into(),
            ],
        )
        .map_err(|_| Error::Jvm(JvmError::MethodCall))?;

        Ok(())
    }

    fn issue(
        &self,
        name: &[u8],
        description: &[u8],
        quantity: i64,
        decimals: i32,
        is_reissuable: bool,
    ) -> Result<Vec<u8>> {
        let mut env = self
            .jvm
            .attach_current_thread()
            .map_err(|_| Error::Jvm(JvmError::AttachCurrentThread))?;

        let name = env
            .byte_array_from_slice(name)
            .map_err(|_| Error::Jvm(JvmError::NewByteArray))?;

        let description = env
            .byte_array_from_slice(description)
            .map_err(|_| Error::Jvm(JvmError::NewByteArray))?;

        let result = env
            .call_method(
                self.jvm_callback.clone(),
                "issue",
                "([B[BJIZ)[B",
                &[
                    JValue::Object(&name.into()),
                    JValue::Object(&description.into()),
                    quantity.into(),
                    decimals.into(),
                    is_reissuable.into(),
                ],
            )
            .map_err(|_| Error::Jvm(JvmError::MethodCall))?
            .l()
            .map_err(|_| Error::Jvm(JvmError::ReceiveObject))?;

        let bytes = env
            .convert_byte_array(<JObject<'_> as Into<JByteArray>>::into(result))
            .map_err(|_| Error::Jvm(JvmError::ByteArrayConversion))?;

        Ok(bytes.to_vec())
    }

    fn burn(&self, asset_id: &[u8], amount: i64) -> Result<()> {
        let mut env = self
            .jvm
            .attach_current_thread()
            .map_err(|_| Error::Jvm(JvmError::AttachCurrentThread))?;

        let asset_id = env
            .byte_array_from_slice(asset_id)
            .map_err(|_| Error::Jvm(JvmError::NewByteArray))?;

        env.call_method(
            self.jvm_callback.clone(),
            "burn",
            "([BJ)V",
            &[JValue::Object(&asset_id.into()), amount.into()],
        )
        .map_err(|_| Error::Jvm(JvmError::MethodCall))?;

        Ok(())
    }

    fn reissue(&self, asset_id: &[u8], amount: i64, is_reissuable: bool) -> Result<()> {
        let mut env = self
            .jvm
            .attach_current_thread()
            .map_err(|_| Error::Jvm(JvmError::AttachCurrentThread))?;

        let asset_id = env
            .byte_array_from_slice(asset_id)
            .map_err(|_| Error::Jvm(JvmError::NewByteArray))?;

        env.call_method(
            self.jvm_callback.clone(),
            "reissue",
            "([BJZ)V",
            &[
                JValue::Object(&asset_id.into()),
                amount.into(),
                is_reissuable.into(),
            ],
        )
        .map_err(|_| Error::Jvm(JvmError::MethodCall))?;

        Ok(())
    }

    fn lease(&self, recipient: &[u8], amount: i64) -> Result<Vec<u8>> {
        let mut env = self
            .jvm
            .attach_current_thread()
            .map_err(|_| Error::Jvm(JvmError::AttachCurrentThread))?;

        let recipient = env
            .byte_array_from_slice(recipient)
            .map_err(|_| Error::Jvm(JvmError::NewByteArray))?;

        let result = env
            .call_method(
                self.jvm_callback.clone(),
                "lease",
                "([BJ)[B",
                &[JValue::Object(&recipient.into()), amount.into()],
            )
            .map_err(|_| Error::Jvm(JvmError::MethodCall))?
            .l()
            .map_err(|_| Error::Jvm(JvmError::ReceiveObject))?;

        let bytes = env
            .convert_byte_array(<JObject<'_> as Into<JByteArray>>::into(result))
            .map_err(|_| Error::Jvm(JvmError::ByteArrayConversion))?;

        Ok(bytes.to_vec())
    }

    fn cancel_lease(&self, lease_id: &[u8]) -> Result<()> {
        let mut env = self
            .jvm
            .attach_current_thread()
            .map_err(|_| Error::Jvm(JvmError::AttachCurrentThread))?;

        let lease_id = env
            .byte_array_from_slice(lease_id)
            .map_err(|_| Error::Jvm(JvmError::NewByteArray))?;

        env.call_method(
            self.jvm_callback.clone(),
            "cancelLease",
            "([B)V",
            &[JValue::Object(&lease_id.into())],
        )
        .map_err(|_| Error::Jvm(JvmError::MethodCall))?;

        Ok(())
    }

    fn get_block_timestamp(&self) -> Result<i64> {
        let mut env = self
            .jvm
            .attach_current_thread()
            .map_err(|_| Error::Jvm(JvmError::AttachCurrentThread))?;

        let result = env
            .call_method(self.jvm_callback.clone(), "getBlockTimestamp", "()J", &[])
            .map_err(|_| Error::Jvm(JvmError::MethodCall))?
            .j()
            .map_err(|_| Error::Jvm(JvmError::ReceiveLong))?;

        Ok(result)
    }

    fn get_block_height(&self) -> Result<i64> {
        let mut env = self
            .jvm
            .attach_current_thread()
            .map_err(|_| Error::Jvm(JvmError::AttachCurrentThread))?;

        let result = env
            .call_method(self.jvm_callback.clone(), "getBlockHeight", "()J", &[])
            .map_err(|_| Error::Jvm(JvmError::MethodCall))?
            .j()
            .map_err(|_| Error::Jvm(JvmError::ReceiveLong))?;

        Ok(result)
    }

    fn get_tx_sender(&self) -> Result<Vec<u8>> {
        let mut env = self
            .jvm
            .attach_current_thread()
            .map_err(|_| Error::Jvm(JvmError::AttachCurrentThread))?;

        let result = env
            .call_method(self.jvm_callback.clone(), "getTxSender", "()[B", &[])
            .map_err(|_| Error::Jvm(JvmError::MethodCall))?
            .l()
            .map_err(|_| Error::Jvm(JvmError::ReceiveObject))?;

        let bytes = env
            .convert_byte_array(<JObject<'_> as Into<JByteArray>>::into(result))
            .map_err(|_| Error::Jvm(JvmError::ByteArrayConversion))?;

        Ok(bytes.to_vec())
    }

    fn get_tx_payments(&self) -> Result<i32> {
        let mut env = self
            .jvm
            .attach_current_thread()
            .map_err(|_| Error::Jvm(JvmError::AttachCurrentThread))?;

        let result = env
            .call_method(self.jvm_callback.clone(), "getTxPayments", "()I", &[])
            .map_err(|_| Error::Jvm(JvmError::MethodCall))?
            .i()
            .map_err(|_| Error::Jvm(JvmError::ReceiveInt))?;

        Ok(result)
    }

    fn get_tx_payment_asset_id(&self, number: i32) -> Result<Vec<u8>> {
        let mut env = self
            .jvm
            .attach_current_thread()
            .map_err(|_| Error::Jvm(JvmError::AttachCurrentThread))?;

        let result = env
            .call_method(
                self.jvm_callback.clone(),
                "getTxPaymentAssetId",
                "(I)[B",
                &[number.into()],
            )
            .map_err(|_| Error::Jvm(JvmError::MethodCall))?
            .l()
            .map_err(|_| Error::Jvm(JvmError::ReceiveObject))?;

        let bytes = env
            .convert_byte_array(<JObject<'_> as Into<JByteArray>>::into(result))
            .map_err(|_| Error::Jvm(JvmError::ByteArrayConversion))?;

        Ok(bytes.to_vec())
    }

    fn get_tx_payment_amount(&self, number: i32) -> Result<i64> {
        let mut env = self
            .jvm
            .attach_current_thread()
            .map_err(|_| Error::Jvm(JvmError::AttachCurrentThread))?;

        let result = env
            .call_method(
                self.jvm_callback.clone(),
                "getTxPaymentAmount",
                "(I)J",
                &[number.into()],
            )
            .map_err(|_| Error::Jvm(JvmError::MethodCall))?
            .j()
            .map_err(|_| Error::Jvm(JvmError::ReceiveLong))?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jni::sys::jint;

    #[test]
    fn test_error() {
        assert_eq!(JvmError::AttachCurrentThread as jint, 200);
    }
}
