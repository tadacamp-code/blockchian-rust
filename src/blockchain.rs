
use core::str;
use std::collections::HashMap;

use crate::{block::*, transaction::Transaction};
use crate::errors::Result;
use crate::{block, constant::*};
use bincode::{deserialize, serialize};
use log::info;
use sled::IVec;

#[derive(Debug)]
pub struct Blockchain{
    current_hash: String,
    db: sled::Db,
}

pub struct BlockchainIter<'a>{
    current_hash: String,
    bc: &'a Blockchain,
}


impl Blockchain {
    pub fn new() -> Result<Blockchain>{
       info!("open blockchain");

       let db = sled::open("data/blocks")?;
       let hash = db
        .get("LAST")?
        .expect("MUST create a new block database first");

        info!("Found block database");
        let lasthash = String::from_utf8(hash.to_vec())?;
        Ok(Blockchain { 
            current_hash: lasthash.clone(),
             db })
    }

    pub fn create_blockchain(address: String) -> Result<Blockchain>{
        info!("Creating new Blockchain");

        let db = sled::open("data/blocks")?;
        info!("Creating new database");

        let cbtx = Transaction::new_coinbase(address, String::from(GENESIS_COINBASE_DATA))?;
        let genesis= Block::new_genesis_block(cbtx);
        db.insert(genesis.get_hash(),bincode::serialize(&genesis)?)?;
        db.insert("LAST",genesis.get_hash().as_bytes())?;

        let bc = Blockchain{
            current_hash: genesis.get_hash(),
            db,
        };
        bc.db.flush()?;
        Ok((bc))
    }

    pub fn add_block(&mut self, data: String) -> Result<()>{
        let lasthash = self.db.get("LAST")?.unwrap();
        let new_block = Block::new_block(data, String::from_utf8(lasthash.to_vec())?, TARGET_HEXS)?;
        //self.blocks.push(new_block);
        self.db.insert(new_block.get_hash(), bincode::serialize(&new_block)?)?;
        self.db.insert("LAST", new_block.get_hash().as_bytes())?;
        self.current_hash = new_block.get_hash();
        Ok(())
    }

    fn find_unspent_transactions(self, address: &str) -> Vec<Transaction>{
        let mut spent_TXOs:HashMap<String,Vec<i32>> = HashMap::new();
        let mut unspent_TXs: Vec<Transaction> = Vec::new();

        for block in self.iter(){
            for tx in block.get_transaction(){
                for index in 0..tx.vout.len(){
                    if let Some(ids) = spent_TXOs.get(&tx.id){
                        if ids.contains(&(index as i32)){
                            continue;
                        }
                    }
                    if tx.vout[index].can_be_unlock_with(address){
                        unspent_TXs.push(tx.to_owned());
                    }
                }

                if !tx.is_coinbase(){
                    for i in &tx.vin{
                        if i.can_unlock_output_with(address){
                            match spent_TXOs.get_mut(&i.txid) {
                                Some(v) => {
                                    v.push(i.vout);
                                }
                                None => {
                                    spent_TXOs.insert(i.txid.clone(), vec![i.vout]);
                                }
                            }
                        }
                    }
                }
            }
        }

        unspent_TXs
    }

    pub fn find_UTXO(&self, address: &str) -> Vec<TXOutput>{
        let mut utxos = Vec::<TXOutput>::new();
        let unspend_TXs = self.find_unspent_transactions(address);
        
        for tx in unspent_TXs{
            for out in &tx.vout{
                if out.can_be_unlock_with(&address){
                    utxos.push(out.clone)
                }
            }
        }
        utxos
    }

    pub fn find_spendable_outputs(
        &self,
        address: &str,
        amount: i32,
    ) -> (i32,HashMap<String,Vec<i32>>){

    }

    pub fn iter(&self) -> BlockchainIter{
        BlockchainIter { 
            current_hash: self.current_hash.clone(), 
            bc:&self, 
        }
    }
}

impl<'a> Iterator for BlockchainIter<'a> {
    type Item = Block;
    fn next(&mut self) -> Option<Self::Item>{
        if let Ok(encode_block) = self.bc.db.get(&self.current_hash){
           return match encode_block {
               Some(b) => {
                    if let Ok(block) = bincode::deserialize::<Block>(&b){
                        self.current_hash = block.get_prev_hash();
                        Some(block)
                    }else {
                        None
                    }
               }
               None => None
           };
        }
        None
    }
}
