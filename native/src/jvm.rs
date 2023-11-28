use crate::{stack::Stack, Error, Result};
use jni::objects::{JByteArray, JObject, JValue};

/// A primitive java type.
/// L - Object
/// Z - Boolean
/// B - Byte
/// I - Integer
/// J - Long
/// V - Void

#[derive(Clone, Copy, Debug, PartialEq)]
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
    /// Failed to receive byte
    ReceiveByte = 208,
    /// Failed to receive integer
    ReceiveInt = 209,
    /// Failed to receive long
    ReceiveLong = 210,
}

pub trait Jvm {
    fn get_chain_id(&self) -> Result<i8>;
    fn get_bytecode(&self, contract_id: &[u8]) -> Result<Vec<u8>>;
    fn add_payments(&self, contract_id: &[u8], payments: &[u8]) -> Result<()>;
    fn get_storage(&self, address: &[u8], key: &[u8]) -> Result<Vec<u8>>;
    fn set_storage(&self, contract_id: &[u8], value: &[u8]) -> Result<()>;
    fn get_balance(&self, asset_id: &[u8], address: &[u8]) -> Result<i64>;
    fn transfer(
        &self,
        contract_id: &[u8],
        asset_id: &[u8],
        recipient: &[u8],
        amount: i64,
    ) -> Result<()>;
    fn issue(
        &self,
        contract_id: &[u8],
        name: &[u8],
        description: &[u8],
        quantity: i64,
        decimals: i32,
        is_reissuable: bool,
    ) -> Result<Vec<u8>>;
    fn burn(&self, contract_id: &[u8], asset_id: &[u8], amount: i64) -> Result<()>;
    fn reissue(
        &self,
        contract_id: &[u8],
        asset_id: &[u8],
        amount: i64,
        is_reissuable: bool,
    ) -> Result<()>;
    fn lease(&self, contract_id: &[u8], recipient: &[u8], amount: i64) -> Result<Vec<u8>>;
    fn cancel_lease(&self, contract_id: &[u8], lease_id: &[u8]) -> Result<()>;
    fn get_block_timestamp(&self) -> Result<i64>;
    fn get_block_height(&self) -> Result<i64>;
    fn get_tx_sender(&self) -> Result<Vec<u8>>;
    fn get_tx_payments(&self, contract_id: &[u8]) -> Result<i32>;
    fn get_tx_payment_asset_id(&self, contract_id: &[u8], number: i32) -> Result<Vec<u8>>;
    fn get_tx_payment_amount(&self, contract_id: &[u8], number: i32) -> Result<i64>;
}

macro_rules! env {
    ($self:expr) => {{
        $self
            .jvm
            .attach_current_thread()
            .map_err(|_| Error::Jvm(JvmError::AttachCurrentThread))?
    }};
}

macro_rules! byte_array {
    ($env:expr, $value:expr) => {{
        $env.byte_array_from_slice($value)
            .map_err(|_| Error::Jvm(JvmError::NewByteArray))?
    }};
}

// Implementing the JVM call
impl Jvm for Stack {
    fn get_chain_id(&self) -> Result<i8> {
        let mut env = env!(self);

        let result = env
            .call_method(self.jvm_callback.clone(), "getChainId", "()B", &[])
            .map_err(|_| Error::Jvm(JvmError::MethodCall))?
            .b()
            .map_err(|_| Error::Jvm(JvmError::ReceiveByte))?;

        Ok(result)
    }

    fn get_bytecode(&self, contract_id: &[u8]) -> Result<Vec<u8>> {
        let mut env = env!(self);

        let contract_id = byte_array!(env, contract_id);

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

    fn add_payments(&self, contract_id: &[u8], payments: &[u8]) -> Result<()> {
        let mut env = env!(self);

        let contract_id = byte_array!(env, contract_id);
        let payments = byte_array!(env, payments);

        env.call_method(
            self.jvm_callback.clone(),
            "addPayments",
            "([B[B)V",
            &[
                JValue::Object(&contract_id.into()),
                JValue::Object(&payments.into()),
            ],
        )
        .map_err(|_| Error::Jvm(JvmError::MethodCall))?;

        Ok(())
    }

    fn get_storage(&self, address: &[u8], key: &[u8]) -> Result<Vec<u8>> {
        let mut env = env!(self);

        let address = byte_array!(env, address);
        let key = byte_array!(env, key);

        let result = env
            .call_method(
                self.jvm_callback.clone(),
                "getStorage",
                "([B[B)[B",
                &[JValue::Object(&address.into()), JValue::Object(&key.into())],
            )
            .map_err(|_| Error::Jvm(JvmError::MethodCall))?
            .l()
            .map_err(|_| Error::Jvm(JvmError::ReceiveObject))?;

        let bytes = env
            .convert_byte_array(<JObject<'_> as Into<JByteArray>>::into(result))
            .map_err(|_| Error::Jvm(JvmError::ByteArrayConversion))?;

        Ok(bytes.to_vec())
    }

    fn set_storage(&self, contract_id: &[u8], value: &[u8]) -> Result<()> {
        let mut env = env!(self);

        let contract_id = byte_array!(env, contract_id);
        let value = byte_array!(env, value);

        env.call_method(
            self.jvm_callback.clone(),
            "setStorage",
            "([B[B)V",
            &[
                JValue::Object(&contract_id.into()),
                JValue::Object(&value.into()),
            ],
        )
        .map_err(|_| Error::Jvm(JvmError::MethodCall))?;

        Ok(())
    }

    fn get_balance(&self, asset_id: &[u8], address: &[u8]) -> Result<i64> {
        let mut env = env!(self);

        let asset_id = byte_array!(env, asset_id);
        let address = byte_array!(env, address);

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

    fn transfer(
        &self,
        contract_id: &[u8],
        asset_id: &[u8],
        recipient: &[u8],
        amount: i64,
    ) -> Result<()> {
        let mut env = env!(self);

        let contract_id = byte_array!(env, contract_id);
        let asset_id = byte_array!(env, asset_id);
        let recipient = byte_array!(env, recipient);

        env.call_method(
            self.jvm_callback.clone(),
            "transfer",
            "([B[B[BJ)V",
            &[
                JValue::Object(&contract_id.into()),
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
        contract_id: &[u8],
        name: &[u8],
        description: &[u8],
        quantity: i64,
        decimals: i32,
        is_reissuable: bool,
    ) -> Result<Vec<u8>> {
        let mut env = env!(self);

        let contract_id = byte_array!(env, contract_id);
        let name = byte_array!(env, name);
        let description = byte_array!(env, description);

        let result = env
            .call_method(
                self.jvm_callback.clone(),
                "issue",
                "([B[B[BJIZ)[B",
                &[
                    JValue::Object(&contract_id.into()),
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

    fn burn(&self, contract_id: &[u8], asset_id: &[u8], amount: i64) -> Result<()> {
        let mut env = env!(self);

        let contract_id = byte_array!(env, contract_id);
        let asset_id = byte_array!(env, asset_id);

        env.call_method(
            self.jvm_callback.clone(),
            "burn",
            "([B[BJ)V",
            &[
                JValue::Object(&contract_id.into()),
                JValue::Object(&asset_id.into()),
                amount.into(),
            ],
        )
        .map_err(|_| Error::Jvm(JvmError::MethodCall))?;

        Ok(())
    }

    fn reissue(
        &self,
        contract_id: &[u8],
        asset_id: &[u8],
        amount: i64,
        is_reissuable: bool,
    ) -> Result<()> {
        let mut env = env!(self);

        let contract_id = byte_array!(env, contract_id);
        let asset_id = byte_array!(env, asset_id);

        env.call_method(
            self.jvm_callback.clone(),
            "reissue",
            "([B[BJZ)V",
            &[
                JValue::Object(&contract_id.into()),
                JValue::Object(&asset_id.into()),
                amount.into(),
                is_reissuable.into(),
            ],
        )
        .map_err(|_| Error::Jvm(JvmError::MethodCall))?;

        Ok(())
    }

    fn lease(&self, contract_id: &[u8], recipient: &[u8], amount: i64) -> Result<Vec<u8>> {
        let mut env = env!(self);

        let contract_id = byte_array!(env, contract_id);
        let recipient = byte_array!(env, recipient);

        let result = env
            .call_method(
                self.jvm_callback.clone(),
                "lease",
                "([B[BJ)[B",
                &[
                    JValue::Object(&contract_id.into()),
                    JValue::Object(&recipient.into()),
                    amount.into(),
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

    fn cancel_lease(&self, contract_id: &[u8], lease_id: &[u8]) -> Result<()> {
        let mut env = env!(self);

        let contract_id = byte_array!(env, contract_id);
        let lease_id = byte_array!(env, lease_id);

        env.call_method(
            self.jvm_callback.clone(),
            "cancelLease",
            "([B[B)V",
            &[
                JValue::Object(&contract_id.into()),
                JValue::Object(&lease_id.into()),
            ],
        )
        .map_err(|_| Error::Jvm(JvmError::MethodCall))?;

        Ok(())
    }

    fn get_block_timestamp(&self) -> Result<i64> {
        let mut env = env!(self);

        let result = env
            .call_method(self.jvm_callback.clone(), "getBlockTimestamp", "()J", &[])
            .map_err(|_| Error::Jvm(JvmError::MethodCall))?
            .j()
            .map_err(|_| Error::Jvm(JvmError::ReceiveLong))?;

        Ok(result)
    }

    fn get_block_height(&self) -> Result<i64> {
        let mut env = env!(self);

        let result = env
            .call_method(self.jvm_callback.clone(), "getBlockHeight", "()J", &[])
            .map_err(|_| Error::Jvm(JvmError::MethodCall))?
            .j()
            .map_err(|_| Error::Jvm(JvmError::ReceiveLong))?;

        Ok(result)
    }

    fn get_tx_sender(&self) -> Result<Vec<u8>> {
        let mut env = env!(self);

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

    fn get_tx_payments(&self, contract_id: &[u8]) -> Result<i32> {
        let mut env = env!(self);

        let contract_id = byte_array!(env, contract_id);

        let result = env
            .call_method(
                self.jvm_callback.clone(),
                "getTxPayments",
                "([B)I",
                &[JValue::Object(&contract_id.into())],
            )
            .map_err(|_| Error::Jvm(JvmError::MethodCall))?
            .i()
            .map_err(|_| Error::Jvm(JvmError::ReceiveInt))?;

        Ok(result)
    }

    fn get_tx_payment_asset_id(&self, contract_id: &[u8], number: i32) -> Result<Vec<u8>> {
        let mut env = env!(self);

        let contract_id = byte_array!(env, contract_id);

        let result = env
            .call_method(
                self.jvm_callback.clone(),
                "getTxPaymentAssetId",
                "([BI)[B",
                &[JValue::Object(&contract_id.into()), number.into()],
            )
            .map_err(|_| Error::Jvm(JvmError::MethodCall))?
            .l()
            .map_err(|_| Error::Jvm(JvmError::ReceiveObject))?;

        let bytes = env
            .convert_byte_array(<JObject<'_> as Into<JByteArray>>::into(result))
            .map_err(|_| Error::Jvm(JvmError::ByteArrayConversion))?;

        Ok(bytes.to_vec())
    }

    fn get_tx_payment_amount(&self, contract_id: &[u8], number: i32) -> Result<i64> {
        let mut env = env!(self);

        let contract_id = byte_array!(env, contract_id);

        let result = env
            .call_method(
                self.jvm_callback.clone(),
                "getTxPaymentAmount",
                "([BI)J",
                &[JValue::Object(&contract_id.into()), number.into()],
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
