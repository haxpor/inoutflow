use crate::deserialize::{de_string_to_numeric, de_string_to_U256, de_string_to_bool};

// also re-export U256 to other modules.
pub use primitive_types::U256;

/// Type of bscscan.com's API request
pub enum BSCApiResponseType {
    NormalTransaction,
    InternalTransaction
}

/// Structure that holds information from API response from bscscan.com
/// of normal transaction
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]  // source JSON response is in camelCase except
                                    // 'txreceipt_status' which we explicitly `rename` it.
pub struct BSCNormalTransactionResponseSuccessVariantResult {
    #[serde(deserialize_with = "de_string_to_numeric")]
    pub block_number: u64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    #[serde(rename = "timeStamp")]
    pub timestamp: u128,

    pub hash: String,

    pub nonce: String,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub transaction_index: u64,

    pub from: String,

    pub to: String,

    #[serde(deserialize_with = "de_string_to_U256")]
    pub value: U256,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub gas: u64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub gas_price: u64,

    #[serde(deserialize_with = "de_string_to_bool")]
    pub is_error: bool,

    #[serde(rename = "txreceipt_status")]
    pub txreceipt_status: String,

    pub input: String,

    pub contract_address: String,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub cumulative_gas_used: u64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub gas_used: u64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub confirmations: u32,
}

/// Structure that holds information from API response from bscscan.com
/// of internal transaction
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BSCInternalTransactionResponseSuccessVariantResult {
    #[serde(deserialize_with = "de_string_to_numeric")]
    pub block_number: u64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    #[serde(rename = "timeStamp")]
    pub timestamp: u128,

    pub hash: String,

    pub from: String,

    pub to: String,

    #[serde(deserialize_with = "de_string_to_U256")]
    pub value: U256,

    pub contract_address: String,

    pub input: String,

    // this is how to escape reserved keyword to use as identifier
    pub r#type: Option<String>,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub gas: u64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub gas_used: u64,

    pub trace_id: Option<String>,

    #[serde(deserialize_with = "de_string_to_bool")]
    pub is_error: bool,

    pub err_code: Option<String>
}

/// Generic result as returned from `result` field from API response from bscscan.com
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
pub enum GenericBSCTransactionResponseResult<T> {
    Success(Vec::<T>),
    Failed(String)
}

/// Common structure which has shared fields for API response from bscscan.com.
#[derive(Debug, serde::Deserialize)]
pub struct BSCTransactionResponse<T> {
    pub status: String,
    pub message: String,
    pub result: GenericBSCTransactionResponseResult::<T>,
}

/// Trait to satisfy implementing generic handling function for multiple API response
/// within one function.
pub trait CompatibleTransactionResponse<T> {
    fn status(&self) -> &str;
    fn message(&self) -> &str;
    fn result(&self) -> GenericBSCTransactionResponseResult::<T>;
}

/// Implementation of `CompatibleTransactionResponse` for
/// `BSCNormalTransactionResponseSuccessVariantResult`.
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

/// Implementation of `CompatibleTransactionResponse` for
/// `BSCInternalTransactionResponseSuccessVariantResult`.
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

/// List of possible this program's error types.
#[derive(Debug)]
pub enum AppError {
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
