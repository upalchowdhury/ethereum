use alloy_rpc_types::{Signature, Transaction};






pub enum TransactionType {
    AtoA(Address,Address,U256),
    ContractDep(Address,Bytes),
    ContractCall(Address,Address,Bytes),
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
