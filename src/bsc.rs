use crate::types::*;

use isahc::prelude::*;
use url::Url;

pub fn get_list_normal_transactions(address: &str) -> Result<Vec::<BSCNormalTransactionResponseSuccessVariantResult>, AppError>
{
    type ResultType = BSCNormalTransactionResponseSuccessVariantResult;
    type JsonType = BSCTransactionResponse::<ResultType>;

    get_list_transactions::<ResultType, JsonType>(BSCApiResponseType::NormalTransaction, address)
}

pub fn get_list_internal_transactions(address: &str) -> Result<Vec::<BSCInternalTransactionResponseSuccessVariantResult>, AppError>
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
        if let Err(_) = url {
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
                                    return Err(AppError::ErrorApiResponse(format!("un-expected error for success case ({msg})", msg=msg)));
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
