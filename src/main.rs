mod types;
mod deserialize;
mod impls;
mod bsc;

// use re-exported type from `types` module.
use types::U256;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		eprintln!("{}", types::AppError::ErrorNotEnoughArgumentsSuppliedAtCommandline(None));
		eprintln!("Usage: {pkg_name} <target_address>", pkg_name=env!("CARGO_PKG_NAME"));
        std::process::exit(1);
	}

    let mut target_address = args[1].to_owned();
    target_address.make_ascii_lowercase();

    // FIXME: might use U256 to make it super accurate
    let mut bnb_balance: f64 = 0_f64;

    // get normal transactions
    {
        let txs_res = bsc::get_list_normal_transactions(target_address.as_str());
        if let Err(e) = txs_res {
            eprintln!("{}", e);
            std::process::exit(1);
        }

        if let Ok(txs) = txs_res {
            let bnb_outflow: U256 = txs.iter().filter(|tx| tx.from == target_address).fold(U256::zero(), |acc, tx| acc + tx.value);
            let bnb_inflow: U256 = txs.iter().filter(|tx| tx.to == target_address).fold(U256::zero(), |acc, tx| acc + tx.value);

            let scale = 10_f64.powf(18.0);

            let bnb_outflow_f = bnb_outflow.to_f64_lossy() / scale;
            let bnb_inflow_f = bnb_inflow.to_f64_lossy() / scale;

            // add feature "fp-conversion" for primitive-types crate to use to_f64_lossy()
            println!("Found {} transactions!", txs.len());
            println!("- BNB outflow: {} BNBs", bnb_outflow_f);
            println!("- BNB inflow: {} BNBs", bnb_inflow_f);
            println!("- BNB balance: {} BNBs", bnb_inflow_f - bnb_outflow_f);

            bnb_balance = bnb_inflow_f - bnb_outflow_f;
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
            let bnb_outflow: U256 = txs.iter().filter(|tx| tx.from == target_address).fold(U256::zero(), |acc, tx| acc + tx.value);
            let bnb_inflow: U256 = txs.iter().filter(|tx| tx.to == target_address).fold(U256::zero(), |acc, tx| acc + tx.value);

            let scale = 10_f64.powf(18.0);

            let bnb_outflow_f = bnb_outflow.to_f64_lossy() / scale;
            let bnb_inflow_f = bnb_inflow.to_f64_lossy() / scale;

            // add feature "fp-conversion" for primitive-types crate to use to_f64_lossy()
            println!("Found {} internal transactions!", txs.len());
            println!("- BNB outflow: {} BNBs", bnb_outflow_f);
            println!("- BNB inflow: {} BNBs", bnb_inflow_f);
            println!("- BNB balance: {} BNBs", bnb_inflow_f - bnb_outflow_f);

            bnb_balance = bnb_balance + (bnb_inflow_f - bnb_outflow_f);
        }
    }

    println!("");
    println!("Total balance: {} BNBs", bnb_balance);
}
