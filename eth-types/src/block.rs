
use alloy_primitives::{address, fixed_bytes, Address, FixedBytes, I256, U256, U64,Bloom, Bytes, B256, B64 };
use serde::{Deserialize,Serialize};
use alloy_rpc_types::{
    other::OtherFields,
    request::{TransactionInput, TransactionRequest as CallRequest},
    AccessList, AccessListItem, Signature, Transaction,
};




#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub number: U64,
    pub difficulty: U256,
    pub gas_limit: U64,
    pub hash: H256,
    pub logs_bloom: Bytes,
    pub nonce: String,
    pub parent_hash: H256,
    pub receipts_root: H256,
    pub size: U64,
    pub state_root: H256,
    pub timestamp: U64,
    pub transactions: Transactions,
    pub transactions_root: H256,
    pub uncles: Vec<H256>,
}

impl Block{
    fn new () -> Self {
        Self{

            number: value.block_number().as_u64().into(),
            base_fee_per_gas: U256::from_little_endian(
                &value.base_fee_per_gas().to_bytes_le(),
            ),
            difficulty: U256::from(0),
            gas_limit: value.gas_limit().as_u64().into(),
            gas_used: value.gas_used().as_u64().into(),
            hash: H256::from_slice(value.block_hash()),
            logs_bloom: value.logs_bloom().to_vec().into(),
            parent_hash: H256::from_slice(value.parent_hash()),
            receipts_root: H256::from_slice(value.receipts_root()),
            state_root: H256::from_slice(value.state_root()),
            timestamp: value.timestamp().as_u64().into(),
            total_difficulty: 0.into(),
            transactions: Transactions::Full(txs),
            nonce: empty_nonce,
            size: 0.into(),
            transactions_root: H256::default(),
            uncles: vec![],

        }
    }
}

#[derive(Deserialize,Debug,Clone)]
pub enum Transactions {
    Transactionhashes(Vec<H256>),
    Fulltransactiondata(Vec<Transaction>),
}


impl Default for Transactions {
    fn default()->Self{
        Self::Fulltransactiondata(Vec::new())
    }
}


impl Transactions {
    pub fn hashses(&self) -> Vec<H256>{
        match self{
            Self::transactionhashses(hashes) => hashes.clone(),
            Self::Fulltransactiondata(txs) => txz.iter().map(|tx| tx.hash).collect(),
        }
    }
}

impl Serialize for Transactions {
    fn serialize<S>(&self,s:S) -> std::result::Result<S::Ok,S::Error>
    where
        S:serde::Serializer,

        {match self{
            Transactions::transactionhashses(hashes) => {
                let mut seq = s.serialize_seq(Some(hashes.len()))?;
                for hash in hashes {
                    seq.serialize_element(&hash)?;
                }
                seq.end()
            }


            Transactions::Fulltransactiondata(txs) => {
                let mut seq = s.serialize_seq(Some(txs.len()))?;
                for tx in txs {
                    seq.serialize_element(&tx)?;
                }
                seq.end()
            }
        }
    
    
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BlockTag {
    Last,
    Mined,
    Number(U64)
}

impl Display for BlockTag {
    fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        let formatted = match self{
            Self::Last => "last".to_string(),
            Self::Mined => "mined".to_string(),
            Self::Number(num) => num.to_string(),
        };
        write!(f, "{formatted}")
    }
}

pub type Blocknumber = BlockTag::Number(U64);

impl deref for Blocknumber{
    type Target = U64;

    fn deref(&self) -> &U64 {
        &self.0
    }
}

impl From<i32> for Blocknumber {
    fn from(value:i32) -> Blocknumber {
        let val = U64::from(value);
        Blocknumber(val);
    }
}

impl TryFrom<&str> for Blocknumber {
    type Error = TypeError;

    fn try_from(value:&str) -> Result<Self> {
        let val = hex_to_u64(value.to_string())?;
        Ok(Blocknumber(val))
    }
}