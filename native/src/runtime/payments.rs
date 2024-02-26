/// Structure allowing to accumulate payments for calling a contract function.
pub struct Payments {
    bytes: Vec<u8>,
    count: u16,
}

impl Default for Payments {
    fn default() -> Self {
        Self::new()
    }
}

impl Payments {
    /// Initialization of an empty set of function payments.
    pub fn new() -> Self {
        Self {
            bytes: vec![],
            count: 0,
        }
    }

    /// Getting byte representation of function payments.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = self.count.to_be_bytes().to_vec();
        bytes.extend(self.bytes.clone());
        bytes
    }

    /// Adding an payment to call the contract function.
    pub fn push(&mut self, asset_id: &[u8], amount: i64) {
        if asset_id.is_empty() {
            self.bytes.push(0);
        } else {
            self.bytes.push(1);
            self.bytes.extend_from_slice(asset_id);
        }

        self.bytes.extend_from_slice(&amount.to_be_bytes());
        self.count += 1;
    }

    /// Reset payments
    pub fn reset(&mut self) {
        self.bytes = vec![];
        self.count = 0;
    }
}
