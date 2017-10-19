use std::hash::{Hash,Hasher};
use std::io;
use std::collections::hash_map::DefaultHasher;
use std::collections::VecDeque;
use std::fmt;
//#[derive(Clone)]

///The 'Block' struct.  
pub struct Block{
    ///The block number.
    num: i32,
    ///The actual data of the block. Currently can only be a 'String', future work will generalize this to arbitrary data.  
    data: String,
    ///The hash of the previous 'Block' in the 'Chain'. If this is the first block, 'prevHash' is set to 0.
    prevHash: u64,
}

///Putting the 'Chain' in blockchain. 
pub struct Chain{
    ///The only element of this struct, a vecdeque of 'Block's. 
    lst: VecDeque<Block>
}


impl Hash for Block{
    ///Custom hash method for 'Block'.
    ///
    ///The hash of a block is the hash of the concatenation of all the block's fields.
    fn hash<H: Hasher>(&self, state: &mut H){
        let mut s = String::new();
        s+=&self.num.to_string();
        s+=&self.data;
        s+=&self.prevHash.to_string();
        s.hash(state)
    }
}

impl fmt::Display for Block{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "num: {} \ndata: {} \nprevious hash: {}", self.num, self.data, self.prevHash)
    }
}

impl Block{
    ///Creates a new block.
    ///
    /// Packages a block number, string data, and hash of previous block into a new block.
    /// This method is used primarily for testing purposes.
    pub fn new(n: i32, d: String, prevh: u64)->Block{
        Block{num: n, data: d, prevHash: prevh}
    }

    ///Returns the hash of a block.
    ///
    /// This method contains the basic sequence of calls required to get the actual hash of a struct. It is used when inserting new blocks into a chain, or when validating a chain. 
    pub fn get_hash(&self )->u64{
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}

impl Chain{
    ///Returns a new (empty) blockchain.
    pub fn new()->Chain{
        let list: VecDeque<Block>=VecDeque::new();
        let  this = Chain{lst:list};
        return this;
    }

    ///Inserts a new block into the chain given the data for the new block.
    ///
    /// If this function is called on an empty chain, the genesis block is inserted. Otherwise, we calculate required fields for the new block and push it to the back of the vecdeque. 
    pub fn insert_block(&mut self, dataStr: String){
        let len = self.lst.len();
        if len==0{
            let genesis = Block{num: 0, data:dataStr, prevHash:0};
            self.lst.push_back(genesis);
        }else{
            let prevh = self.lst.back_mut().unwrap().get_hash();
            let newblock = Block{num:len as i32, data:dataStr,prevHash:prevh};
            self.lst.push_back(newblock);
        }
    }

    ///Determines whether a chain is valid.
    ///
    /// A chain is valid if the 'prevHash' value of each block matches the hash of the previous block. If this condition holds for all blocks in the chain, then it is valid.
    pub fn validate_chain(&mut self)->bool{
        for e in self.lst.iter(){
            if e.num>0{
                if e.prevHash != self.lst.get((e.num-1) as usize).unwrap().get_hash(){
                    return false;
                }
            }
        }
        true
    }
    
}
impl fmt::Display for Chain{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let mut s = String::new();
        for e in self.lst.iter(){
            s += &format!("{}", e);
            s += "\n"
        }
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_display(){
        // let mut ch = Chain::new();
        // ch.insert_block("Block1 data".to_string());
        // ch.insert_block("Block2 data".to_string());
        // println!("{}",ch);
        //print_chain_rn();
        let b1 = Block{num: 0, data: "lala".to_string(), prevHash: 0};
        let b2 = Block{num: 1, data: "lolo".to_string(), prevHash: b1.get_hash()};
        println!("{}", b1);
        println!("{}", b2);
    }
    
    #[test]
    fn chain_valid() {
        let mut ch = Chain::new();
        ch.insert_block("Block1 data".to_string());
        ch.insert_block("Block2 data".to_string());
        
        assert_eq!(true, ch.validate_chain());
    }
}


