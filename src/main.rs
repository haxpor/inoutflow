use isahc::prelude::*;
use url::Url;
use serde::{Deserialize, Deserializer};
use primitive_types::*;
use std::env::VarError;

/// Type of bscscan.com's API request
enum BSCApiResponseType {
    NormalTransaction,
    InternalTransaction
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]  // source JSON response is in camelCase except
                                    // 'txreceipt_status' which we explicitly `rename` it.
struct BSCNormalTransactionResponseSuccessVariantResult {
    #[serde(deserialize_with = "de_string_to_numeric")]
    block_number: u64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    time_stamp: u128,    

    hash: String,

    nonce: String,

    #[serde(deserialize_with = "de_string_to_numeric")]
    transaction_index: u64,

    from: String,

    to: String,

    #[serde(deserialize_with = "de_string_to_U256")]
    value: U256,

    #[serde(deserialize_with = "de_string_to_numeric")]
    gas: u64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    gas_price: u64,

    #[serde(deserialize_with = "de_string_to_bool")]
    is_error: bool,

    #[serde(rename = "txreceipt_status")]
    txreceipt_status: String,

    input: String,

    contract_address: String,

    #[serde(deserialize_with = "de_string_to_numeric")]
    cumulative_gas_used: u64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    gas_used: u64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    confirmations: u32,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct BSCInternalTransactionResponseSuccessVariantResult {
    #[serde(deserialize_with = "de_string_to_numeric")]
    block_number: u64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    time_stamp: u128,

    hash: String,

    from: String,

    to: String,

    #[serde(deserialize_with = "de_string_to_U256")]
    value: U256,

    contract_address: String,

    input: String,

    // this is how to escape reserved keyword to use as identifier
    r#type: Option<String>,

    #[serde(deserialize_with = "de_string_to_numeric")]
    gas: u64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    gas_used: u64,

    trace_id: Option<String>,

    #[serde(deserialize_with = "de_string_to_bool")]
    is_error: bool,

    err_code: Option<String>
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
enum GenericBSCTransactionResponseResult<T> {
    Success(Vec::<T>),
    Failed(String)
}

#[derive(Debug, serde::Deserialize)]
struct BSCTransactionResponse<T> {
    status: String,
    message: String,
    result: GenericBSCTransactionResponseResult::<T>,
}

trait CompatibleTransactionResponse<T> {
    fn status(&self) -> &str;
    fn message(&self) -> &str;
    fn result(&self) -> GenericBSCTransactionResponseResult::<T>;
}

impl CompatibleTransactionResponse<BSCNormalTransactionResponseSuccessVariantResult> for BSCTransactionResponse<BSCNormalTransactionResponseSuccessVariantResult>
{
    fn status(&self) -> &str {
        &self.status
    }

    fn message(&self) -> &str {
        &self.message
    }

    fn result(&self) -> GenericBSCTransactionResponseResult::<BSCNormalTransactionResponseSuccessVariantResult> {
        self.result.clone()
    }
}

impl CompatibleTransactionResponse<BSCInternalTransactionResponseSuccessVariantResult> for BSCTransactionResponse<BSCInternalTransactionResponseSuccessVariantResult>
{
    fn status(&self) -> &str {
        &self.status
    }

    fn message(&self) -> &str {
        &self.message
    }

    fn result(&self) -> GenericBSCTransactionResponseResult::<BSCInternalTransactionResponseSuccessVariantResult> {
        self.result.clone()
    }
}

#[derive(Debug)]
enum AppError {
    /// Internal error for generic error combined altogether
    /// Contain optional error message
    ErrorInternalGeneric(Option<String>),

    /// Internal error from parsing Url
    ErrorInternalUrlParsing,

    /// Error in sending HTTP request
    ErrorSendingHttpRequest,

    /// Error due to no api-key defined via environment variable HX_INOUTFLOW_API_KEY
    ErrorNoApiKey,

    /// Api key defined but it is not unicode
    ErrorApiKeyNotUnicode,

    /// Error JSON parsing
    /// Contain optional error message
    ErrorJsonParsing(Option<String>),

    /// Error from Api response back from bscscan.com containing the error message
    ErrorApiResponse(String),

	/// Error not enough arguments supplied at command line
    /// Contain optional message for error.
	ErrorNotEnoughArgumentsSuppliedAtCommandline(Option<String>),
}

// follow the pattern as seen in std::env https://doc.rust-lang.org/src/std/env.rs.html#263-299
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match *self {
            AppError::ErrorInternalGeneric(ref msg) => {
                match msg {
                    Some(msg) => write!(f, "Error internal operation ({})", msg),
                    None => write!(f, "Error internal operation"),
                }
            },
            AppError::ErrorInternalUrlParsing => write!(f, "Error internal from parsing Url"),
            AppError::ErrorSendingHttpRequest => write!(f, "Error in sending HTTP request"),
            AppError::ErrorNoApiKey => write!(f, "Error no api-key defined via environment variable HX_INOUTFLOW_API_KEY"),
            AppError::ErrorApiKeyNotUnicode => write!(f, "Api key is defined, but it is not unicode."),
            AppError::ErrorJsonParsing(ref msg) => {
                match msg {
                    Some(msg) => write!(f, "Error in parsing JSON string ({})", msg),
                    None => write!(f, "Error in parsing JSON string"),
                }
            },
            AppError::ErrorApiResponse(ref msg) => write!(f, "Error api response from bscscan.com: {}", msg),
			AppError::ErrorNotEnoughArgumentsSuppliedAtCommandline(ref msg) => {
                match msg {
                    Some(msg) => write!(f, "Error not enough arguments supplied at commandline ({})", msg),
                    None => write!(f, "Error not enough arguments supplied at commandline.")
                }
            }
        }
    }
}

impl std::error::Error for AppError {}

impl std::convert::From<VarError> for AppError {
    fn from(f: VarError) -> Self {
        match f {
            VarError::NotPresent => AppError::ErrorNoApiKey,
            // NOTE: can also use .. but it has different semantics
            // .. means range of all arguments, although in this case we just have one
            VarError::NotUnicode(_) => AppError::ErrorApiKeyNotUnicode,
        }
    }
}

fn de_string_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>
{
    let buf = String::deserialize(deserializer)?;
    if buf == "1" {
        return Ok(true);
    }
    else {
        return Ok(false);
    }
}

/// Look at example at https://serde.rs/stream-array.html
fn de_string_to_numeric<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: std::str::FromStr + serde::Deserialize<'de>,
    <T as std::str::FromStr>::Err: std::fmt::Display // std::str::FromStr has `Err` type, see https://doc.rust-lang.org/std/str/trait.FromStr.html
{
    let buf = String::deserialize(deserializer)?;
    // convert into serde's custom Error type
    buf.parse::<T>().map_err(serde::de::Error::custom)
}

fn de_string_to_U256<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>
{
    let buf = String::deserialize(deserializer)?;
    U256::from_dec_str(&buf).map_err(serde::de::Error::custom)
}

fn get_list_normal_transactions(address: &str) -> Result<Vec::<BSCNormalTransactionResponseSuccessVariantResult>, AppError>
{
    type ResultType = BSCNormalTransactionResponseSuccessVariantResult;
    type JsonType = BSCTransactionResponse::<ResultType>;

    get_list_transactions::<ResultType, JsonType>(BSCApiResponseType::NormalTransaction, address)
}

fn get_list_internal_transactions(address: &str) -> Result<Vec::<BSCInternalTransactionResponseSuccessVariantResult>, AppError>
{
    type ResultType = BSCInternalTransactionResponseSuccessVariantResult;
    type JsonType = BSCTransactionResponse::<ResultType>;

    get_list_transactions::<ResultType, JsonType>(BSCApiResponseType::InternalTransaction, address)
}

fn get_list_transactions<R, J>(api_req_type: BSCApiResponseType, address: &str) -> Result<Vec::<R>, AppError>
where
    R: serde::de::DeserializeOwned,
    J: CompatibleTransactionResponse::<R> + serde::de::DeserializeOwned
{
    let mut page_number = 1u8;
    let mut is_need_next_page = true;
    const OFFSET: usize = 1000;   // per request will get max txs

    let mut ret_txs: Vec::<R> = Vec::new();

    // VarError would be converted into AppError
    // as we have implemented From<VarError> for AppError
    let api_key = std::env::var("HX_INOUTFLOW_API_KEY")?;

    while is_need_next_page {
        // beware to always use fully qualified here for type of api_req_type
        let action = match &api_req_type {
            BSCApiResponseType::NormalTransaction => "txlist",
            BSCApiResponseType::InternalTransaction => "txlistinternal"
        };
        let raw_url_str = format!("https://api.bscscan.com/api?module=account&action={action}&address={target_address}&startblock=0&endblock=99999999&page={page}&offset={offset}&sort=asc&apikey={api_key}", action=action, target_address=address, api_key=api_key, page=page_number, offset=OFFSET);

        let url = Url::parse(&raw_url_str);
        if let Err(e) = url {
            return Err(AppError::ErrorInternalUrlParsing);
        }

        match isahc::get(url.unwrap().as_str()) {
            Ok(mut res) => {
                // early return for non-200 HTTP returned code
                if res.status() != 200 {
                    return Err(AppError::ErrorApiResponse(format!("Error API resonse, with HTTP {code} returned", code=res.status().as_str())));
                }

                // use the commented line, or just use what isahc provides conveniently
                //match serde_json::from_str::<BSCNormalTransactionResponse>(res.text().unwrap().as_str()) {
                //match res.json::<BSCNormalTransactionResponse>() {
                match res.json::<J>() {
                    Ok(json) => {
                        if json.status() == "1" {
                            // NOTE: unfortunate, we need to extract value from within enum
                            // https://stackoverflow.com/questions/34953711/unwrap-inner-type-when-enum-variant-is-known
                            match json.result() {
                                GenericBSCTransactionResponseResult::Success(mut c) => {
                                    if c.len() == 0 {
                                        is_need_next_page = false;
                                    }
                                    else if c.len() > 0 && c.len() < OFFSET {
                                        ret_txs.append(&mut c);
                                        is_need_next_page = false;
                                    }
                                    else {
                                        ret_txs.append(&mut c);
                                    }
                                },
                                // this case should not happen
                                GenericBSCTransactionResponseResult::Failed(msg) => {
                                    return Err(AppError::ErrorApiResponse("un-expected error for success case".to_string()));
                                }
                            }
                        }
                        else {
                            // exact text as returned when empty "result" is returned
                            if json.message() == "No transactions found" {
                                break;
                            }
                            else {
                                return Err(AppError::ErrorApiResponse(format!("message:{message}", message=json.message())));
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("{:?}", e);
                        return Err(AppError::ErrorJsonParsing(None));
                    }
                }
            },
            Err(_) => {
                return Err(AppError::ErrorSendingHttpRequest);
            }
        }

        if is_need_next_page {
            page_number = page_number + 1;
        }
        else {
            break;
        }
    }

    Ok(ret_txs)
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		eprintln!("{}", AppError::ErrorNotEnoughArgumentsSuppliedAtCommandline(None));
		eprintln!("Usage: {pkg_name} <target_address>", pkg_name=env!("CARGO_PKG_NAME"));
        std::process::exit(1);
	}

    let mut target_address = args[1].to_owned();
    target_address.make_ascii_lowercase();

    // FIXME: might use U256 to make it super accurate
    let mut bnb_balance: f64 = 0_f64;;

    // get normal transactions
    {
        let txs_res = get_list_normal_transactions(target_address.as_str());
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
        let txs_res = get_list_internal_transactions(target_address.as_str());
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
