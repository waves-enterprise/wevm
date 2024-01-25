use crate::error::Result;

/// Interface of interaction with the node.
pub trait Node {
    fn get_chain_id(&self) -> Result<i8>;
    fn get_bytecode(&self, contract_id: &[u8]) -> Result<Vec<u8>>;
    fn add_payments(&self, contract_id: &[u8], payment_id: &[u8], payments: &[u8]) -> Result<()>;
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
        decimals: i64,
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
    fn get_tx_payments(&self, payment_id: &[u8]) -> Result<i64>;
    fn get_tx_payment_asset_id(&self, payment_id: &[u8], number: i64) -> Result<Vec<u8>>;
    fn get_tx_payment_amount(&self, payment_id: &[u8], number: i64) -> Result<i64>;
}
