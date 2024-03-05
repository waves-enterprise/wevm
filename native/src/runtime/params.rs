use crate::runtime::data_entry::DataEntry;

/// Structure allowing to accumulate arguments for calling a contract function.
pub struct Params {
    bytes: Vec<u8>,
    count: u16,
}

impl Default for Params {
    fn default() -> Self {
        Self::new()
    }
}

impl Params {
    /// Initialization of an empty set of function arguments.
    pub fn new() -> Self {
        Self {
            bytes: vec![],
            count: 0,
        }
    }

    /// Getting byte representation of function arguments.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = self.count.to_be_bytes().to_vec();
        bytes.extend(self.bytes.clone());
        bytes
    }

    /// Adding an argument to call the contract function.
    pub fn push(&mut self, value: DataEntry) {
        self.bytes.extend(value.serialize(None));
        self.count += 1;
    }

    /// Reset params
    pub fn reset(&mut self) {
        self.bytes = vec![];
        self.count = 0;
    }
}
