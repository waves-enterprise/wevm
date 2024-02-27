use crate::{
    error::{Error, JvmError, Result},
    node::Node,
    vm::Vm,
};
use jni::objects::{JByteArray, JObject, JValue};

// A primitive java type.
// L - Object
// Z - Boolean
// B - Byte
// I - Integer
// J - Long
// V - Void

macro_rules! env {
    ($self:expr) => {{
        match &$self.jvm {
            Some(jvm) => jvm
                .attach_current_thread()
                .map_err(|_| Error::Jvm(JvmError::AttachCurrentThread))?,
            None => return Err(Error::Jvm(JvmError::JvmNotFound)),
        }
    }};
}

macro_rules! byte_array {
    ($env:expr, $value:expr) => {{
        $env.byte_array_from_slice($value)
            .map_err(|_| Error::Jvm(JvmError::NewByteArray))?
    }};
}

macro_rules! jvm_callback {
    ($jvm_callback:expr) => {
        match $jvm_callback {
            Some(jvm_callback) => jvm_callback,
            None => return Err(Error::Jvm(JvmError::JvmCallbackNotFound)),
        }
    };
}

// Implementing the JVM call
impl Node for Vm {
    fn get_chain_id(&self) -> Result<i8> {
        let mut env = env!(self);

        let result = env
            .call_method(jvm_callback!(&self.jvm_callback), "getChainId", "()B", &[])
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
                jvm_callback!(&self.jvm_callback),
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

    fn add_payments(&self, contract_id: &[u8], payment_id: &[u8], payments: &[u8]) -> Result<()> {
        let mut env = env!(self);

        let contract_id = byte_array!(env, contract_id);
        let payment_id = byte_array!(env, payment_id);
        let payments = byte_array!(env, payments);

        env.call_method(
            jvm_callback!(&self.jvm_callback),
            "addPayments",
            "([B[B[B)V",
            &[
                JValue::Object(&contract_id.into()),
                JValue::Object(&payment_id.into()),
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
                jvm_callback!(&self.jvm_callback),
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
            jvm_callback!(&self.jvm_callback),
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
                jvm_callback!(&self.jvm_callback),
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
            jvm_callback!(&self.jvm_callback),
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
        decimals: i64,
        is_reissuable: bool,
    ) -> Result<Vec<u8>> {
        let mut env = env!(self);

        let contract_id = byte_array!(env, contract_id);
        let name = byte_array!(env, name);
        let description = byte_array!(env, description);

        let result = env
            .call_method(
                jvm_callback!(&self.jvm_callback),
                "issue",
                "([B[B[BJJZ)[B",
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
            jvm_callback!(&self.jvm_callback),
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
            jvm_callback!(&self.jvm_callback),
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
                jvm_callback!(&self.jvm_callback),
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
            jvm_callback!(&self.jvm_callback),
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
            .call_method(
                jvm_callback!(&self.jvm_callback),
                "getBlockTimestamp",
                "()J",
                &[],
            )
            .map_err(|_| Error::Jvm(JvmError::MethodCall))?
            .j()
            .map_err(|_| Error::Jvm(JvmError::ReceiveLong))?;

        Ok(result)
    }

    fn get_block_height(&self) -> Result<i64> {
        let mut env = env!(self);

        let result = env
            .call_method(
                jvm_callback!(&self.jvm_callback),
                "getBlockHeight",
                "()J",
                &[],
            )
            .map_err(|_| Error::Jvm(JvmError::MethodCall))?
            .j()
            .map_err(|_| Error::Jvm(JvmError::ReceiveLong))?;

        Ok(result)
    }

    fn tx(&self, field: &[u8]) -> Result<Vec<u8>> {
        let mut env = env!(self);

        let field = byte_array!(env, field);

        let result = env
            .call_method(
                jvm_callback!(&self.jvm_callback),
                "tx",
                "([B)[B",
                &[JValue::Object(&field.into())],
            )
            .map_err(|_| Error::Jvm(JvmError::MethodCall))?
            .l()
            .map_err(|_| Error::Jvm(JvmError::ReceiveObject))?;

        let bytes = env
            .convert_byte_array(<JObject<'_> as Into<JByteArray>>::into(result))
            .map_err(|_| Error::Jvm(JvmError::ByteArrayConversion))?;

        Ok(bytes.to_vec())
    }

    fn get_tx_payments(&self, payment_id: &[u8]) -> Result<i64> {
        let mut env = env!(self);

        let payment_id = byte_array!(env, payment_id);

        let result = env
            .call_method(
                jvm_callback!(&self.jvm_callback),
                "getTxPayments",
                "([B)J",
                &[JValue::Object(&payment_id.into())],
            )
            .map_err(|_| Error::Jvm(JvmError::MethodCall))?
            .j()
            .map_err(|_| Error::Jvm(JvmError::ReceiveInt))?;

        Ok(result)
    }

    fn get_tx_payment_asset_id(&self, payment_id: &[u8], number: i64) -> Result<Vec<u8>> {
        let mut env = env!(self);

        let payment_id = byte_array!(env, payment_id);

        let result = env
            .call_method(
                jvm_callback!(&self.jvm_callback),
                "getTxPaymentAssetId",
                "([BJ)[B",
                &[JValue::Object(&payment_id.into()), number.into()],
            )
            .map_err(|_| Error::Jvm(JvmError::MethodCall))?
            .l()
            .map_err(|_| Error::Jvm(JvmError::ReceiveObject))?;

        let bytes = env
            .convert_byte_array(<JObject<'_> as Into<JByteArray>>::into(result))
            .map_err(|_| Error::Jvm(JvmError::ByteArrayConversion))?;

        Ok(bytes.to_vec())
    }

    fn get_tx_payment_amount(&self, payment_id: &[u8], number: i64) -> Result<i64> {
        let mut env = env!(self);

        let payment_id = byte_array!(env, payment_id);

        let result = env
            .call_method(
                jvm_callback!(&self.jvm_callback),
                "getTxPaymentAmount",
                "([BJ)J",
                &[JValue::Object(&payment_id.into()), number.into()],
            )
            .map_err(|_| Error::Jvm(JvmError::MethodCall))?
            .j()
            .map_err(|_| Error::Jvm(JvmError::ReceiveLong))?;

        Ok(result)
    }
}
