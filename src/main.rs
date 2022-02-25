mod types;
mod deserialize;
mod impls;
mod bsc;

// use re-exported type from `types` module.
use types::bsc_types::U256;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		eprintln!("{}", types::AppError::ErrorNotEnoughArgumentsSuppliedAtCommandline(None));
		eprintln!("Usage: {pkg_name} <target_address>", pkg_name=env!("CARGO_PKG_NAME"));
        std::process::exit(1);
	}

    let mut target_address = args[1].to_owned();
    target_address.make_ascii_lowercase();

    let mut amount_normal_transactions = 0;
    let mut amount_internal_transactions = 0;

    // get normal transactions
    {
        let txs_res = bsc::get_list_normal_transactions(target_address.as_str());
        if let Err(e) = txs_res {
            eprintln!("{}", e);
            std::process::exit(1);
        }

        if let Ok(txs) = txs_res {
            amount_normal_transactions = txs.len();

            let bnb_outflow: U256 = txs.iter().filter(|tx| (tx.from == target_address) && !tx.is_error)
                .fold(U256::zero(), |acc, tx| acc + tx.value);
            let bnb_inflow: U256 = txs.iter().filter(|tx| (tx.to == target_address) && !tx.is_error)
                .fold(U256::zero(), |acc, tx| acc + tx.value);

            let bnb_outflow_f = bnb_outflow.to_f64_lossy() / bsc::BNB_SCALE_F;
            let bnb_inflow_f = bnb_inflow.to_f64_lossy() / bsc::BNB_SCALE_F;

            // add feature "fp-conversion" for primitive-types crate to use to_f64_lossy()
            println!("Found {} txs!", txs.len());
            println!("- BNB outflow: {} BNBs", bnb_outflow_f);
            println!("- BNB inflow: {} BNBs", bnb_inflow_f);
            println!("- Net in/out balance: {} BNBs", bnb_inflow_f - bnb_outflow_f);
        }
    }

    println!("");

    // get internal transactions
    {
        let txs_res = bsc::get_list_internal_transactions(target_address.as_str());
        if let Err(e) = txs_res {
            eprintln!("{}", e);
            std::process::exit(1);
        }

        if let Ok(txs) = txs_res {
            amount_internal_transactions = txs.len();

            let bnb_outflow: U256 = txs.iter().filter(|tx| tx.from == target_address && !tx.is_error)
                .fold(U256::zero(), |acc, tx| acc + tx.value);
            let bnb_inflow: U256 = txs.iter().filter(|tx| tx.to == target_address && !tx.is_error)
                .fold(U256::zero(), |acc, tx| acc + tx.value);

            let bnb_outflow_f = bnb_outflow.to_f64_lossy() / bsc::BNB_SCALE_F;
            let bnb_inflow_f = bnb_inflow.to_f64_lossy() / bsc::BNB_SCALE_F;

            // add feature "fp-conversion" for primitive-types crate to use to_f64_lossy()
            println!("Found {} internal txs!", txs.len());
            println!("- BNB outflow: {} BNBs", bnb_outflow_f);
            println!("- BNB inflow: {} BNBs", bnb_inflow_f);
            println!("- Net in/out balance: {} BNBs", bnb_inflow_f - bnb_outflow_f);
        }
    }

    println!("");
    println!("Total {} txs", amount_normal_transactions + amount_internal_transactions);

    // get balance of the address
    // NOTE: we probably can prove our way to manually calculate the total balance
    // out of normal, internal, and fees in transferring BEP-20 token or others.
    // For now, we just utilize API to just retrieve the balance right away.
    // We can prove the concept later.
    {
        let balance_res = bsc::get_balance_address(target_address.as_str());
        if let Err(e) = balance_res {
            eprintln!("{}", e);
            std::process::exit(1);
        }

        if let Ok(balance) = balance_res {
            println!("Total balance: {} BNBs", balance.to_f64_lossy() / bsc::BNB_SCALE_F);
        }
    }
}
