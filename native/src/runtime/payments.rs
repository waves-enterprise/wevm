/// Structure allowing to accumulate payments for calling a contract function.
pub struct Payments(pub Vec<(Vec<u8>, i64)>);

impl Default for Payments {
    fn default() -> Self {
        Self::new()
    }
}

impl Payments {
    /// Initialization of an empty set of function payments.
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Getting byte representation of function payments.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        for item in &self.0 {
            let asset_id = &item.0;
            let amount = item.1;

            if asset_id.is_empty() {
                bytes.push(0);
            } else {
                bytes.push(1);
                bytes.extend_from_slice(asset_id);
            }

            bytes.extend_from_slice(&amount.to_be_bytes());
        }

        let mut result = (self.0.len() as u16).to_be_bytes().to_vec();
        result.extend(bytes);
        result
    }

    /// Adding an payment to call the contract function.
    pub fn push(&mut self, asset_id: &[u8], amount: i64) {
        self.0.push((asset_id.to_vec(), amount));
    }

    /// Reset payments
    pub fn reset(&mut self) {
        self.0 = vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BYTES: [u8; 52] = [
        0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 42, 1, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3,
        3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 0, 0, 0, 0, 0, 0, 0, 24,
    ];

    #[test]
    fn test_serialize_payments() {
        let mut payments = Payments::default();

        payments.push(&[], 42);
        payments.push(
            &[
                1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7,
                1, 3, 3, 7,
            ],
            24,
        );

        assert_eq!(payments.as_bytes(), BYTES.to_vec());
    }
}
