pub mod Silent_threshold_enc;
#[derive(Clone, Copy, Debug)]
pub enum DataType {
    U32(u32),
    U64(u64),
    U128(u128),
    Bytes32([u8; 32]),
    #[allow(dead_code)]
    Address([u8; 48]),
}

impl DataType {
    pub fn to_vec(&self) -> Vec<u8> {
        match self {
            DataType::U128(value) => value.to_le_bytes().to_vec(),
            DataType::Bytes32(bytes) => bytes.to_vec(),
            DataType::Address(bytes) => bytes.to_vec(),
            DataType::U32(value) => value.to_le_bytes().to_vec(),
            DataType::U64(value) => value.to_le_bytes().to_vec(),
        }
    }
}

fn to_array_32(input: Vec<u8>) -> [u8; 32] {
    let mut array = [0u8; 32]; // Default all to zeros
    let len = input.len().min(32); // Avoid out-of-bounds
    array[..len].copy_from_slice(&input[..len]);
    array
}

#[derive(Clone, Debug, Copy)]
pub enum EncryptionSchemeInputs {
    SilentThreshold( DataType, usize),
  
}

pub trait EncryptionScheme {
    fn scheme_name(&self) -> String;
    fn get_inputs(
        &self,
        data_type: DataType,
        n: usize,
    ) -> EncryptionSchemeInputs;
    fn encrypt(&self, encryption_scheme_inputs: EncryptionSchemeInputs);
    fn decrypt(&self, encryption_scheme_inputs: EncryptionSchemeInputs);
}
