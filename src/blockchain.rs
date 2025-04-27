use super::*;
use crate::block::*; // 导入block模块
use std::fmt;
use bincode::{deserialize, serialize};
use sled;//导入sled键值对数据库

// 定义区块链结构和操作

//区块链结构定义
#[derive(Debug)]
pub struct Blockchain 
{
    tip: String, //最新区块的哈希值
    //blocks: Vec<Block>, // 区块链由一系列块组成
    current_hash: String, //当前遍历区块的哈希值
    db: sled::Db, //数据库实例
}

impl Blockchain 
{
    // 创建一个新的区块链   
    pub fn new() -> Result<Blockchain> {
        info!("Creating new blockchain");
        // 打开数据库
        let db = sled::open("data/blocks")?;
        // 检查数据库中是否存在最后一个区块的哈希值
        match db.get("LAST")? {
            Some(hash) => {
                // 如果存在，将其转换为字符串，创建并返回一个对应的区块链实例
                info!("Found block database");
                let lasthash = String::from_utf8(hash.to_vec())?;
                Ok(Blockchain {
                    tip: lasthash.clone(),
                    current_hash: lasthash,
                    db,
                })
            }
            None => {
                // 如果不存在，创建一个新的创世区块，并将其添加到数据库中
                info!("Creating new block database");
                let block = Block::new_genesis_block();
                //将区块序列化后存储到数据库中
                db.insert(block.get_hash(), serialize(&block)?)?;
                db.insert("LAST", block.get_hash().as_bytes())?;
                // 创建并返回一个对应的区块链实例
                let bc = Blockchain {
                    tip: block.get_hash(),
                    current_hash: block.get_hash(),
                    db,
                };
                bc.db.flush()?;
                Ok(bc)
            }
        }
    }

    pub fn add_block(&mut self, data: String) -> Result<()> 
    {
        info!("add new block to the chain");
        //获取最新区块的哈希值
        let lasthash = self.db.get("LAST")?.unwrap();
        //创建新区块
        let newblock = Block::new_block(data, String::from_utf8(lasthash.to_vec())?)?;
        //将新区块添加到数据库中
        self.db.insert(newblock.get_hash(), serialize(&newblock)?)?;
        //更新最新区块的哈希值
        self.db.insert("LAST", newblock.get_hash().as_bytes())?;
        // 刷新数据库，确保所有数据都已写入磁盘
        self.db.flush()?;

        //更新内存中的指针
        self.tip = newblock.get_hash();
        self.current_hash = newblock.get_hash();

        Ok(())
    }
}

//区块链遍历
impl Iterator for Blockchain {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        //尝试从数据库中获取当前区块
        if let Ok(encoded_block) = self.db.get(&self.current_hash) {
            return match encoded_block {
                //如果获取成功，将其反序列化为 Block 类型，并更新当前区块的哈希值
                Some(b) => {
                    if let Ok(block) = deserialize::<Block>(&b) {
                        self.current_hash = block.get_prev_hash();
                        Some(block)
                    } else {
                        None
                    }
                }
                None => None,
            };
        }
        None
    }
}
// ... existing code ...

// 为 Blockchain 实现 Display trait
impl fmt::Display for Blockchain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "区块链信息:")?;
        
        // 创建一个临时的区块链副本用于遍历
        let mut temp_chain = Blockchain {
            tip: self.tip.clone(),
            current_hash: self.tip.clone(),
            db: self.db.clone(),
        };
        
        let mut blocks = Vec::new();
        // 使用迭代器收集所有区块
        for block in &mut temp_chain {
            blocks.push(block);
        }
        
        writeln!(f, "共有 {} 个区块", blocks.len())?;
        writeln!(f, "------------------------")?;
        
        // 反转区块顺序，使其从最早的区块开始显示
        blocks.reverse();
        for (i, block) in blocks.iter().enumerate() {
            writeln!(f, "区块 #{}", i)?;
            write!(f, "{}", block)?;
            if i < blocks.len() - 1 {
                writeln!(f, "\n------------------------")?;
            }
        }
        
        Ok(())
    }
}