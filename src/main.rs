use ::bscscan::bscscan;
use ::bscscan::environ::Context;
use ::bscscan::prelude::*;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		eprintln!("Usage: {pkg_name} <target_address>", pkg_name=env!("CARGO_PKG_NAME", "inoutflow-bsc"));
        std::process::exit(1);
	}

    let mut target_address = args[1].to_owned();
    target_address.make_ascii_lowercase();

    // create bscscan's context
    let ctx = Context { api_key: std::env::var("HX_INOUTFLOW_API_KEY").expect("required 'HX_INOUTFLOW_API_KEY' environment variable to be defined") };

    // get normal transactions
    {
        let txs_res = bscscan::accounts().get_list_normal_transactions(&ctx, target_address.as_str());
        if let Err(e) = txs_res {
            eprintln!("{}", e);
            std::process::exit(1);
        }

        if let Ok(txs) = txs_res {
            if txs.len() == 0 {
                println!("Found 0 txs!");
            }
            else {
                let bnb_outflow: U256 = txs.iter().filter(|tx| (tx.from == target_address) && !tx.is_error)
                    .fold(U256::zero(), |acc, tx| acc + tx.value);
                let bnb_inflow: U256 = txs.iter().filter(|tx| (tx.to == target_address) && !tx.is_error)
                    .fold(U256::zero(), |acc, tx| acc + tx.value);

                let bnb_outflow_f = bnb_outflow.to_f64_lossy() / bscscan::BNB_SCALE_F;
                let bnb_inflow_f = bnb_inflow.to_f64_lossy() / bscscan::BNB_SCALE_F;

                // add feature "fp-conversion" for primitive-types crate to use to_f64_lossy()
                println!("Found {} txs!", txs.len());
                println!("- BNB outflow: {} BNBs", bnb_outflow_f);
                println!("- BNB inflow: {} BNBs", bnb_inflow_f);
                println!("- Net in/out balance: {} BNBs", bnb_inflow_f - bnb_outflow_f);
            }
        }
    }

    println!("");

    // get internal transactions
    {
        let txs_res = bscscan::accounts().get_list_internal_transactions(&ctx, target_address.as_str());
        if let Err(e) = txs_res {
            eprintln!("{}", e);
            std::process::exit(1);
        }

        if let Ok(txs) = txs_res {
            if txs.len() == 0 {
                println!("Found 0 internal txs!");
            }
            else {
                let bnb_outflow: U256 = txs.iter().filter(|tx| tx.from == target_address && !tx.is_error)
                    .fold(U256::zero(), |acc, tx| acc + tx.value);
                let bnb_inflow: U256 = txs.iter().filter(|tx| tx.to == target_address && !tx.is_error)
                    .fold(U256::zero(), |acc, tx| acc + tx.value);

                let bnb_outflow_f = bnb_outflow.to_f64_lossy() / bscscan::BNB_SCALE_F;
                let bnb_inflow_f = bnb_inflow.to_f64_lossy() / bscscan::BNB_SCALE_F;

                // add feature "fp-conversion" for primitive-types crate to use to_f64_lossy()
                println!("Found {} internal txs!", txs.len());
                println!("- BNB outflow: {} BNBs", bnb_outflow_f);
                println!("- BNB inflow: {} BNBs", bnb_inflow_f);
                println!("- Net in/out balance: {} BNBs", bnb_inflow_f - bnb_outflow_f);
            }
        }
    }

    println!("");

    // get balance of the address
    // NOTE: we probably can prove our way to manually calculate the total balance
    // from in/out flow of normal and internal transactions, and gas fees in `normal` transactions.
    // See README.md at `Technical Tips` section for more detail.
    {
        let balance_res = bscscan::accounts().get_balance_address(&ctx, target_address.as_str());
        if let Err(e) = balance_res {
            eprintln!("{}", e);
            std::process::exit(1);
        }

        if let Ok(balance) = balance_res {
            println!("Total balance: {} BNBs", balance.to_f64_lossy() / bscscan::BNB_SCALE_F);
        }
    }
}
