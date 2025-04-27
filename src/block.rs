use super::*; //导入上层模块的所有内容，上级是main.rs
use bincode::serialize; //用于将数据序列化为二进制格式
use crypto::digest::Digest; //用于哈希算法
use crypto::sha2::Sha256; //使用SHA-256哈希算法 
use std::time::SystemTime; //用于获取系统时间 

//定义区块结构和相关方法

//定义区块结构，包括时间戳，前一个区块的哈希值，当前区块哈希值，交易数据
#[derive(Debug)]  //表示该结构体可以被打印,自动实现Debug trait，可以通过{:?}格式化输出 
pub struct Block
{
    
    timestamp:u128, //时间戳
    data: String, //数据
    prev_block_hash: String, //前一个区块的哈希值
    hash: String, //当前区块哈希值
}

//impl表示实现Block结构体的方法
impl Block
{
    //set_hash()方法，用于计算当前区块的哈希值
    //&self表示该方法接收一个可变的Block结构体引用作为参数
    //Result<()>表示该方法返回一个Result类型，其中泛型参数为()，表示该方法没有返回值
    pub fn set_hash(&mut self) -> Result<()>
    {
        self.timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_millis(); //获取当前时间戳
        //clone()方法用于创建一个新的String对象，该对象与原对象内容相同
        let content = (self.data.clone(), self.timestamp); //将区块数据和时间戳组合成元组
        let bytes = serialize(&content)?; //将元组序列化为二进制格式 
        let mut hasher = Sha256::new(); //创建一个SHA-256哈希计算器对象 
        hasher.input(&bytes[..]); //将二进制数据输入到哈希计算器中 
        self.hash = hasher.result_str(); //计算哈希值并将其赋值给当前区块的hash字段 
        Ok(()) //返回一个空的Result对象，表示方法执行成功   
    }

    pub fn get_hash(&self) -> String 
    {
        self.hash.clone() 
    }

    pub fn new_block(data: String, prev_block_hash: String) -> Result<Block>
    {
        //创建一个新的Block对象
        let mut block = Block {
            timestamp: 0,
            data,
            prev_block_hash,
            hash: String::new(),
        };
        block.set_hash()?; //计算哈希值
        Ok(block) //返回Block对象
    }
     
    //创建创世区块
    pub fn new_genesis_block() -> Block
    {
        //创世块交易数据为"Genesis Block"，前一个区块哈希值为空字符串，unwrap()方法用于将Result对象转换为Block对象
        Block::new_block(String::from("Genesis Block"),String::new()).unwrap()
    }
}
