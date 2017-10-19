use std::hash::{Hash,Hasher};
use std::io;
use std::collections::hash_map::DefaultHasher;
use std::collections::VecDeque;

pub struct Block{
    num: i32,
    data: String,
    prevHash: u64,
}

pub struct Chain{
    lst: VecDeque<Block>
}

impl Hash for Block{

    fn hash<H: Hasher>(&self, state: &mut H){
        let mut s = String::new();
        s+=&self.num.to_string();
        s+=&self.data;
        s+=&self.prevHash.to_string();
        s.hash(state)
    }
    
}

impl Block{
    pub fn getHash(&self )->u64{
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}

impl Chain{
    pub fn new()->Chain{
        let list: VecDeque<Block>=VecDeque::new();
        let mut this = Chain{lst:list};
        return this;
    }

    pub fn insert_block(&mut self, dataStr: String){        
        if self.lst.len()==0{
            let genesis = Block{num: 0, data:dataStr, prevHash:0};
            self.lst.push_back(genesis);
        }else{
            let  prevb = self.lst.back_mut().unwrap();
            let prevh = prevb.getHash();
            let newblock = Block{num:self.lst.len() as i32, data:dataStr,prevHash:prevh};
            self.lst.push_back(newblock);
        }
    }

    pub fn validate_chain(&mut self)->bool{
        for e in self.lst.iter(){
            if e.num>0{
                if e.prevHash != self.lst.get((e.num-1) as usize).unwrap().getHash(){
                    return false;
                }
            }
        }
        true
    }
}




