use super::*;
use crate::blockchain::*;
use crate::transaction::*;
use crate::wallets::*;
use bitcoincash_addr::Address;
use clap::{App, Arg};
use std::process::exit;

pub struct Cli {}

impl Cli {
    pub fn new() -> Cli {
        Cli {}
    }

    pub fn run(&mut self) -> Result<()> {
        info!("run app");
        let matches = App::new("blockchain-demo")
            .version("0.1")
            .author("yunwei37. 1067852565@qq.com")
            .about("reimplement blockchain_go in rust: a simple blockchain for learning")
            .subcommand(App::new("printchain").about("print all the chain blocks"))
            .subcommand(App::new("createwallet").about("create a wallet"))
            .subcommand(App::new("listaddresses").about("list all addresses"))
            .subcommand(
                App::new("getbalance")
                    .about("get balance in the blockchain")
                    .arg(Arg::from_usage(
                        "<address> 'The address to get balance for'",
                    )),
            )
            .subcommand(App::new("createblockchain").about("create blockchain").arg(
                Arg::from_usage("<address> 'The address to send genesis block reward to'"),
            ))
            .subcommand(
                App::new("send")
                    .about("send in the blockchain")
                    .arg(Arg::from_usage("<from> 'Source wallet address'"))
                    .arg(Arg::from_usage("<to> 'Destination wallet address'"))
                    .arg(Arg::from_usage("<amount> 'Amount to send'")),
            )
            .get_matches();

        if let Some(ref matches) = matches.subcommand_matches("getbalance") {
            if let Some(address) = matches.value_of("address") {
                let pub_key_hash = Address::decode(address).unwrap().body;
                let bc = Blockchain::new()?;
                let utxos = bc.find_UTXO(&pub_key_hash);

                let mut balance = 0;
                for out in utxos {
                    balance += out.value;
                }
                println!("Balance: {}\n", balance);
            }
        }

        if let Some(_) = matches.subcommand_matches("createwallet") {
            let mut ws = Wallets::new()?;
            let address = ws.create_wallet();
            ws.save_all()?;
            println!("success: address {}", address);
        }

        if let Some(_) = matches.subcommand_matches("printchain") {
            let bc = Blockchain::new()?;
            for b in bc.iter() {
                println!("{:#?}", b);
            }
        }

        if let Some(_) = matches.subcommand_matches("listaddresses") {
            let ws = Wallets::new()?;
            let addresses = ws.get_all_addresses();
            println!("addresses: ");
            for ad in addresses {
                println!("{}", ad);
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("createblockchain") {
            if let Some(address) = matches.value_of("address") {
                let address = String::from(address);
                Blockchain::create_blockchain(address.clone())?;
                println!("create blockchain");
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("send") {
            let from = if let Some(address) = matches.value_of("from") {
                address
            } else {
                println!("from not supply!: usage\n{}", matches.usage());
                exit(1)
            };
            let to = if let Some(address) = matches.value_of("to") {
                address
            } else {
                println!("to not supply!: usage\n{}", matches.usage());
                exit(1)
            };
            let amount: i32 = if let Some(amount) = matches.value_of("amount") {
                amount.parse()?
            } else {
                println!("amount in send not supply!: usage\n{}", matches.usage());
                exit(1)
            };

            let mut bc = Blockchain::new()?;
            let tx = Transaction::new_UTXO(from, to, amount, &bc)?;
            let cbtx = Transaction::new_coinbase(from.to_string(), String::from("reward!"))?;

            bc.mine_block(vec![cbtx, tx])?;
            println!("success!");
        }

        Ok(())
    }
}
