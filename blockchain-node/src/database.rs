use std::path::{Path,PathBuf};
use eth_trie::DB as Ethtrie;
use rocksdb::{Options,DB};


const PATH: &str = "./../.tmp";
const DATABASE_NAME: &str = "db";


pub(crate) struct Database {
    db:rocksdb::DB,
}


impl Ethtrie for Database {
    fn get(&self,key:&[u8]) -> Result<Option<Vec<u8>>> {
        let value = self.db.get(key).map_err(|_| Error)?;
        Ok(value);
    }

    fn insert(&self,key:&[u8]) -> Result<Option<Vec<u8>>> {
        self.db.put(key,value).map_err(|_| Error)?;
        Ok(())
    }
}