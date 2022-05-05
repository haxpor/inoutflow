mod types;
mod util;

use ::evmscan::evmscan;
use ::evmscan::environ::Context;
use ::evmscan::prelude::*;
use clap::Parser;
use types::*;
use util::*;

fn main() {
	let cmd_args = CommandlineArgs::parse();

    // validate value of chain flag option
    let chain_value = cmd_args.chain.to_lowercase();
    let mut chain: Option<ChainType> = None;
    if chain_value == "bsc" {
        chain = Some(ChainType::BSC);
    }
    else if chain_value == "ethereum" {
        chain = Some(ChainType::Ethereum);
    }
    else if chain_value == "polygon" {
        chain = Some(ChainType::Polygon);
    }
    // NOTE: no need for else case here as clap crate handles non-valid values
    // for us.
    
    // validate the required argument; address
    if !is_address_simplified(&cmd_args.address) {
        eprintln!("Error: input address is malformed. Make sure to prefix with '0x' and has 40 characters in length (exclude `0x`).");
        std::process::exit(1);
    }

    // make it ascii-lowercased for input address
    let mut target_address = cmd_args.address.to_owned();
    target_address.make_ascii_lowercase();

    // get token name
    let native_token_name = get_native_token_name(chain.unwrap());

    // create a context
    let ctx = Context::create(chain.unwrap(), select_apikey(chain.unwrap()));

    // get normal transactions
    {
        let txs_res = evmscan::accounts().get_list_normal_transactions(&ctx, target_address.as_str());
        if let Err(e) = txs_res {
            eprintln!("{}", e);
            std::process::exit(1);
        }

        if let Ok(txs) = txs_res {
            if txs.len() == 0 {
                println!("Found 0 txs!");
            }
            else {
                let ntoken_outflow: U256 = txs.iter().filter(|tx| (tx.from == target_address) && !tx.is_error)
                    .fold(U256::zero(), |acc, tx| acc + tx.value);
                let ntoken_inflow: U256 = txs.iter().filter(|tx| (tx.to == target_address) && !tx.is_error)
                    .fold(U256::zero(), |acc, tx| acc + tx.value);

                let ntoken_outflow_f = ntoken_outflow.to_f64_lossy() / evmscan::NATIVE_TOKEN_SCALE_F;
                let ntoken_inflow_f = ntoken_inflow.to_f64_lossy() / evmscan::NATIVE_TOKEN_SCALE_F;

                // add feature "fp-conversion" for primitive-types crate to use to_f64_lossy()
                println!("Found {} txs!", txs.len());
                println!("- {} outflow: {} {}s", native_token_name, ntoken_outflow_f, native_token_name);
                println!("- {} inflow: {} {}s", native_token_name, ntoken_inflow_f, native_token_name);
                println!("- Net in/out balance: {} {}s", ntoken_inflow_f - ntoken_outflow_f, native_token_name);
            }
        }
    }

    println!("");

    // get internal transactions
    {
        let txs_res = evmscan::accounts().get_list_internal_transactions(&ctx, target_address.as_str());
        if let Err(e) = txs_res {
            eprintln!("{}", e);
            std::process::exit(1);
        }

        if let Ok(txs) = txs_res {
            if txs.len() == 0 {
                println!("Found 0 internal txs!");
            }
            else {
                let ntoken_outflow: U256 = txs.iter().filter(|tx| tx.from == target_address && !tx.is_error)
                    .fold(U256::zero(), |acc, tx| acc + tx.value);
                let ntoken_inflow: U256 = txs.iter().filter(|tx| tx.to == target_address && !tx.is_error)
                    .fold(U256::zero(), |acc, tx| acc + tx.value);

                let ntoken_outflow_f = ntoken_outflow.to_f64_lossy() / evmscan::NATIVE_TOKEN_SCALE_F;
                let ntoken_inflow_f = ntoken_inflow.to_f64_lossy() / evmscan::NATIVE_TOKEN_SCALE_F;

                // add feature "fp-conversion" for primitive-types crate to use to_f64_lossy()
                println!("Found {} internal txs!", txs.len());
                println!("- {} outflow: {} {}s", native_token_name, ntoken_outflow_f, native_token_name);
                println!("- {} inflow: {} {}s", native_token_name, ntoken_inflow_f, native_token_name);
                println!("- Net in/out balance: {} {}s", ntoken_inflow_f - ntoken_outflow_f, native_token_name);
            }
        }
    }

    println!("");

    // get balance of the address
    // NOTE: we probably can prove our way to manually calculate the total balance
    // from in/out flow of normal and internal transactions, and gas fees in `normal` transactions.
    // See README.md at `Technical Tips` section for more detail.
    {
        let balance_res = evmscan::accounts().get_balance_address(&ctx, target_address.as_str());
        if let Err(e) = balance_res {
            eprintln!("{}", e);
            std::process::exit(1);
        }

        if let Ok(balance) = balance_res {
            println!("Total balance: {} {}s", balance.to_f64_lossy() / evmscan::NATIVE_TOKEN_SCALE_F, native_token_name);
        }
    }
}
