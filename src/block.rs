use super::*; //导入上层模块的所有内容，上级是main.rs
use bincode::serialize; //用于将数据序列化为二进制格式
use crypto::digest::Digest; //用于哈希算法
use crypto::sha2::Sha256; //使用SHA-256哈希算法 
use std::time::SystemTime; //用于获取系统时间 
use std::fmt; //用于格式化输出
use chrono::{DateTime, Utc}; //用于处理时间和日期
use serde::{Deserialize, Serialize};//用于序列化和反序列化数据

//定义区块结构和相关方法
const TARGET_HEXS: usize = 7; //定义挖矿难度，即哈希值前4位为0
//定义区块结构，包括时间戳，前一个区块的哈希值，当前区块哈希值，交易数据
#[derive(Serialize, Deserialize, Debug, Clone)] 
//Debug表示该结构体可以被打印,自动实现Debug trait，可以通过{:?}格式化输出 
//Clone表示该结构体可以被克隆，自动实现Clone trait
//Serialize表示该结构体可以被序列化，自动实现Serialize trait
//Deserialize表示该结构体可以被反序列化，自动实现Deserialize trait
pub struct Block
{
    timestamp:u128, //时间戳
    data: String, //数据
    prev_block_hash: String, //前一个区块的哈希值
    hash: String, //当前区块哈希值
    nonce: i32, //计数器，不断尝试计算哈希值，直到满足条件
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
    pub fn get_prev_hash(&self) -> String {
        self.prev_block_hash.clone()
    }

    pub fn new_block(data: String, prev_block_hash: String) -> Result<Block>
    {
        let timestamp = SystemTime::now()
           .duration_since(SystemTime::UNIX_EPOCH)?
           .as_millis(); //获取当前时间戳
        //创建一个新的Block对象
        let mut block = Block {
            timestamp,
            data,
            prev_block_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.run_proof_of_work()?; //运行工作量证明算法
        Ok(block) //返回Block对象
    }
     
    //创建创世区块
    pub fn new_genesis_block() -> Block
    {
        //创世块交易数据为"Genesis Block"，前一个区块哈希值为空字符串，unwrap()方法用于将Result对象转换为Block对象
        Block::new_block(String::from("Genesis Block"),String::new()).unwrap()
    }
    
    //准备用于哈希计算的数据,返回序列化后的block数据
    fn prepare_hash_data(&self)->Result<Vec<u8>>
    {
        let content = 
        (
            self.prev_block_hash.clone(),
            self.data.clone(),
            self.timestamp,
            self.nonce,
        );
        let bytes = serialize(&content)?; //将元组序列化为二进制格式
        Ok(bytes) //返回二进制数据
    }
    fn validate(&self) -> Result<bool>
    {
        let data = self.prepare_hash_data()?; //准备用于哈希计算的数据
        let mut hasher = Sha256::new(); //创建一个SHA-256哈希计算器对象
        hasher.input(&data[..]); //将二进制数据输入到哈希计算器中
        let mut vec1: Vec<u8>= Vec::new(); //创建一个空的Vec对象
        vec1.resize(TARGET_HEXS, '0' as u8); //将vec1的长度调整为TARGET_HEXS，并将所有元素初始化为'0'的ASCII码
        Ok(&hasher.result_str()[0..TARGET_HEXS] == String::from_utf8(vec1)?) //判断哈希值的前TARGET_HEXS个字符是否与vec1中的字符相同
    }
    fn run_proof_of_work(&mut self) -> Result<()>
    {
        println!("Mining the block containing \"{}\"\n", self.data); //打印挖矿信息
        while !self.validate()? //循环直到验证通过
        {
            self.nonce += 1; //计数器加1
        }
        //得到挖矿结果，将哈希值赋值给当前区块的hash字段
        let data= self.prepare_hash_data()?; //准备用于哈希计算的数据
        let mut hasher = Sha256::new(); //创建一个SHA-256哈希计算器对象
        hasher.input(&data[..]); //将二进制数据输入到哈希计算器中
        self.hash = hasher.result_str(); //计算哈希值并将其赋值给当前区块的hash字段
        Ok(()) //返回一个空的Result对象，表示方法执行成功
    }
}
// 为 Block 实现 Display trait
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 将毫秒时间戳转换为DateTime<Utc>
        let seconds = (self.timestamp / 1000) as i64;
        let nanoseconds = ((self.timestamp % 1000) * 1_000_000) as u32;
        let datetime = DateTime::<Utc>::from_timestamp(seconds, nanoseconds)
            .unwrap_or_else(|| Utc::now());
        write!(f, "区块信息:\n")?;
        write!(f, "  时间戳: {} ({})\n", self.timestamp, datetime.format("%Y-%m-%d %H:%M:%S%.3f UTC"))?;
        write!(f, "  数据: {}\n", self.data)?;
        write!(f, "  哈希值: {}\n", self.hash)?;
        write!(f, "  前一区块哈希: {}\n", self.prev_block_hash)?;
        write!(f, "  工作量证明计数器: {}", self.nonce)
    }
}