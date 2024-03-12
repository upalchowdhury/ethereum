use alloy_primitives::{address, fixed_bytes, Address, FixedBytes, I256, U256, U64,Bloom, Bytes, B256, B64 };
use serde::{Deserialize,Serialize};
use alloy_rpc_types::{
    other::OtherFields,
    request::{TransactionInput, TransactionRequest as CallRequest},
    AccessList, AccessListItem, Signature, Transaction,
};


pub type Account = Address;

#[derive(Debug,Serialize,Deserialize,PartialEq,Clone)]
pub struct AccountData{
    pub nonce: U256,
    pub balance: U256,
    pub code_hash: Option<Bytes>
}


impl AccountData {
    pub fn new(code_hash:Option<Bytes>) -> Self {
        Self{
            nonce: U256::zero(),
            balance: U256::zero(),
            code_hash,
        }
    }

    pub fn _is_contract(&self) -> bool {
        self.code_hash.is_some()
    }
}