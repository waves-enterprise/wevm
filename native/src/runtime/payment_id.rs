pub struct PaymentId {
    contract_id: Vec<u8>,
    nonce: u64,
}

impl PaymentId {
    pub fn new(contract_id: Vec<u8>, nonce: u64) -> Self {
        Self { contract_id, nonce }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut result = self.contract_id.clone();
        result.extend_from_slice(self.nonce.to_be_bytes().as_slice());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BYTES: [u8; 72] = [
        1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3,
        3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7,
        1, 3, 3, 7, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    #[test]
    fn test_serialize_payment_id() {
        let bytes = vec![
            1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1,
            3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3, 3, 7, 1, 3,
            3, 7, 1, 3, 3, 7,
        ];

        let payment_id = PaymentId::new(bytes, 0);

        assert_eq!(payment_id.as_bytes(), BYTES.to_vec());
    }
}
