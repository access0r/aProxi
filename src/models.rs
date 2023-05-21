use web3::types::Address;

pub struct ContractData {
    pub resolver_address: Address,
    pub resolver_abi: Vec<u8>,
    pub relayer_address: Address,
    pub relayer_abi: Vec<u8>,
}
