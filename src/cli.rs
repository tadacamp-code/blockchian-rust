use clap::builder::Str;
use clap::{Command, arg};
use crate::errors::Result;
use crate::blockchain::Blockchain;

pub struct Cli{
    bc:Blockchain,
}

impl Cli{
    pub fn new() -> Result<Cli>{
        Ok(Cli{
            bc: Blockchain::new()?,
        })
    }

    pub fn run(&mut self) -> Result<()>{
        let matchers = Command::new("blockchain-rust-demo")
            .version("0.1")
            .author("jeff8211@foxmail.com")
            .about("blockchain in rust: a simple blockchain for learning")
            .subcommand(Command::new("printchain")).about("print all the chain blocks")
            .subcommand(
                Command::new("addblock")
                .about("add a block in the blockchain")
                .arg(arg!(<DATA>"'the blockchain data'")),
            )
            .get_matches();
        if let Some(ref matchers) = matchers.subcommand_matches("addblock"){
            if let Some(c) = matchers.get_one::<String>("DATA"){
                self.addblcok(String::from(c))?;
            }else {
                println!("Not printing test lists...");
            }
        }

        if let Some(_) = matchers.subcommand_matches("printchain"){
            self.print_chain()
        }
        Ok(())
    }


    fn addblcok(&mut self, data: String) -> Result<()>{
        self.bc.add_block(data)
    }

    fn print_chain(&mut self){
        for b in &mut self.bc.iter(){
            println!("block: {:#?}",b);
        }
    }
}