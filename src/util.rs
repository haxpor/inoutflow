use regex::Regex;
use ::evmscan::prelude::ChainType;

/// Check whether specified address string is an address.
///
/// This is not full-fledge checking in which it doesn't take into account
/// checking of checksum address.
///
/// # Arguments
/// * `address` - address string to check
pub fn is_address_simplified(address: &str) -> bool {
    let lowercase_address = address.to_lowercase();
    let regex: Regex = Regex::new(r#"^(0x)?[0-9a-f]{40}$"#).unwrap();

    regex.is_match(&lowercase_address)
}

/// Select and return api key for selected chain type.
/// The program needs environment variables as follows to be defined to cover
/// all API platforms which one of them will be used at runtime depending on
/// which chain has been selected.
///
/// * `bsc` - require environment variable `TRACPLS_BSCSCAN_APIKEY`
/// * `ethereum` - require environment variable `TRACPLS_ETHERSCAN_APIKEY`
/// * `polygon` - require environment variable `TRACPLS_POLYGONSCAN_APIKEY`
///
/// If such environment variable after selected has not defined yet, then
/// this function will panic.
///
/// # Arguments
/// * `chain` - chain type
pub fn select_apikey(chain: ChainType) -> String {
    match chain {
        ChainType::BSC => std::env::var("INOUTFLOW_BSCSCAN_APIKEY").expect("Required environment variable 'INOUTFLOW_BSCSCAN_APIKEY' to be defined"),
        ChainType::Ethereum => std::env::var("INOUTFLOW_ETHERSCAN_APIKEY").expect("Required environment variable 'INOUTFLOW_ETHERSCAN_APIKEY' to be defined"),
        ChainType::Polygon => std::env::var("INOUTFLOW_POLYGONSCAN_APIKEY").expect("Required environment variable 'INOUTFLOW_POLYGONSCAN_APIKEY' to be defined"),
    }
}

/// Get native token name
///
/// # NOTE
/// Probably better to have static string version globally for all native token
/// name here. But this program is simple enough, and we don't need to use this
/// function anywhere else.
///
/// # Arguments
/// * `chain` - chain type
pub fn get_native_token_name(chain: ChainType) -> String {
    match chain {
        ChainType::BSC => "BNB".to_owned(),
        ChainType::Ethereum => "ETH".to_owned(),
        ChainType::Polygon => "MATIC".to_owned(),
    }
}
