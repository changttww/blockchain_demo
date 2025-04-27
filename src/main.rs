mod block;
mod blockchain;
//这样main函数就成了这两个父模块，在谁那里声明，谁就是父模块

//定义一个自定义的Result类型,用于函数返回值
pub type Result<T> = std::result::Result<T, failure::Error>;

use blockchain::*;
//导入用于线程休眠的Duration和sleep函数
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<()>
{
    let mut bc:Blockchain = Blockchain::new();
    //创建一个区块链后休眠10毫秒，确保不同区块有不同时间戳
    sleep(Duration::from_millis(10));
    bc.add_block(String::from("Send 1 BTC to bb"))?; 
    sleep(Duration::from_millis(30));
    bc.add_block(String::from("Send 2 BTC to bb"))?;
    println!("Blockchaain: {:#?}", bc);
    Ok(())
}