use super::*;
use crate::blockchain::*;
use clap::{App, Arg}; //命令行解析库

pub struct Cli {
    bc: Blockchain,
}

impl Cli 
{
    pub fn new() -> Result<Cli> 
    {
        Ok(Cli 
        {
            bc: Blockchain::new()?,
        })
    }

    pub fn run(&mut self) -> Result<()> 
    {
        info!("run app");
        //创建命令行应用
        //new()创建应用，括号里是应用的名称，需要在命令行输入
        let matches = App::new("blockchain-demo")
            .version("0.1")
            .author("cc")
            .about("reimplement blockchain_go in rust: a simple blockchain for learning")
            //子命令，about是对应用的描述
            .subcommand(App::new("printchain").about("print all the chain blocks"))
            //arg()添加参数，from_usage()是参数的用法
            .subcommand(
                App::new("addblock")
                    .about("add a block in the blockchain")
                    .arg(Arg::from_usage("<data> 'the blockchain data'")),
            )
            //get_matches()解析命令行参数
            .get_matches();
            //处理addblock子命令
            //matches.subcommand_matches()获取子命令的参数，存入Some中
        if let Some(ref matches) = matches.subcommand_matches("addblock") {
            //matches.value_of()获取参数的值，存入Some（c）中
            if let Some(c) = matches.value_of("data") 
            {
                //传入获取到的参数值，创建新的Block
                self.addblock(String::from(c))?;
            } else 
            {
                println!("Not printing testing lists...");
            }
        }

        //处理printchain子命令
        if let Some(_) = matches.subcommand_matches("printchain") 
        {
            //打印区块链
            self.print_chain();
        }

        Ok(())
    }

    fn print_chain(&mut self) 
    {
         println!("{}", self.bc);
    }

    fn addblock(&mut self, data: String) -> Result<()> 
    {
        self.bc.add_block(data)
    }
}
