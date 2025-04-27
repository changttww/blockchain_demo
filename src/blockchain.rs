use super::*;
use crate::block::*; // 导入block模块

// 定义区块链结构和操作

//区块链结构定义
#[derive(Debug)]
pub struct Blockchain 
{
    blocks: Vec<Block>, // 区块链由一系列块组成
}

impl Blockchain 
{
    // 创建一个新的区块链
    pub fn new() ->Blockchain
    {
        Blockchain
        {
            blocks: vec![Block::new_genesis_block()]
        }
    }
        

    

    pub fn add_block(&mut self, data: String) -> Result<()>
    {
        let prev = self.blocks.last().unwrap();
        let new_block = Block::new_block(data, prev.get_hash())?; //问号用于处理错误
        self.blocks.push(new_block);
        Ok(())
    }

    
}