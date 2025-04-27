mod block;
mod blockchain;
mod cli;
//这样main函数就成了这两个父模块，在谁那里声明，谁就是父模块

#[macro_use]
use log::*;
//定义一个自定义的Result类型,用于函数返回值
pub type Result<T> = std::result::Result<T, failure::Error>;

use blockchain::*;
//导入用于线程休眠的Duration和sleep函数
use std::thread::sleep;
use std::time::Duration;
use crate::cli::Cli;
use env_logger::Env;

fn main() -> Result<()> {
    env_logger::from_env(Env::default().default_filter_or("warning")).init();

    let mut cli = Cli::new()?;
    cli.run()?;

    Ok(())
}
