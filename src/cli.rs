use clap::{arg, Command};
use crate::blockchain::Blockchain;
use crate::errors::Result;

pub struct Cli {
    bc: Blockchain,
}

impl Cli {
    pub fn new() -> Result<Cli> {
        Ok(Cli {
            bc: Blockchain::new()?,
        })
    }
    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("blockchain-rust-demo")
            .version("0.1")
            .author("behrouz.r.fa@gmail.com")
            .about("blockchain in rust: a simple blockchain for learning")
            .subcommand(Command::new("printchain").about("print all the chain blocks"))
            .subcommand(
                Command::new("addblock")
                    .about("add a block in the blockchain")
                    .arg(arg!(<DATA>" 'the blockchain data'")),
            )
            .get_matches();
        if let Some(ref matches) = matches.subcommand_matches("addblock") {
            if let Some(c) = matches.get_one::<String>("DATA"){
                self.addblock(String::from(c))?;
            } else {
                println!("Not printing testing lists...");
            }
        }

        if let Some(_) = matches.subcommand_matches("printchain") {
            self.print_chain();
        }

        Ok(())
    }

    fn addblock(&mut self, data: String) -> Result<()> {
        self.bc.add_block(data)
    }

    fn print_chain(&mut self) {
        for b in &mut self.bc.iter() {
            println!("block: {:#?}", b);
        }
    }
}
