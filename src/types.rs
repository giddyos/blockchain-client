use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Utxo {
    pub txid: String,
    pub vout: u32,
    pub address: Option<String>,
    pub label: Option<String>,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: String,
    pub amount: f64,
    pub confirmations: u64,
    pub spendable: bool,
    pub solvable: bool,
    pub descriptor: Option<String>,
    pub safe: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionListItem {
    pub txid: String,
    pub address: Option<String>,
    pub category: String,
    pub amount: f64,
    pub label: Option<String>,
    pub vout: Option<u32>,
    pub confirmations: i64,
    pub blockhash: Option<String>,
    pub blockindex: Option<u64>,
    pub blocktime: Option<u64>,
    pub time: u64,
    pub timereceived: u64,
    pub comment: Option<String>,
    pub otheraccount: Option<String>,
    pub fee: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReceivedByAddress {
    pub address: String,
    pub amount: f64,
    pub confirmations: u64,
    pub label: Option<String>,
    #[serde(default)]
    pub txids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptSig {
    pub asm: String,
    pub hex: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptPubKey {
    pub asm: Option<String>,
    pub hex: Option<String>,
    #[serde(rename = "reqSigs")]
    pub req_sigs: Option<u32>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(default)]
    pub addresses: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TxInput {
    pub txid: Option<String>,
    pub vout: Option<u32>,
    #[serde(rename = "scriptSig")]
    pub script_sig: Option<ScriptSig>,
    pub sequence: Option<u64>,
    pub coinbase: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TxOutput {
    pub value: f64,
    pub n: u32,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: ScriptPubKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawTransaction {
    pub txid: String,
    pub hash: Option<String>,
    pub version: Option<i32>,
    pub size: Option<u64>,
    pub vsize: Option<u64>,
    pub weight: Option<u64>,
    pub locktime: Option<u64>,
    #[serde(default)]
    pub vin: Vec<TxInput>,
    #[serde(default)]
    pub vout: Vec<TxOutput>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockchainInfo {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub bestblockhash: String,
    pub difficulty: f64,
    pub mediantime: Option<u64>,
    pub verificationprogress: Option<f64>,
    pub initialblockdownload: Option<bool>,
    pub size_on_disk: Option<u64>,
    pub pruned: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressValidation {
    pub isvalid: bool,
    pub address: Option<String>,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: Option<String>,
    pub isscript: Option<bool>,
    pub iswitness: Option<bool>,
    #[serde(rename = "witness_version")]
    pub witness_version: Option<u32>,
    #[serde(rename = "witness_program")]
    pub witness_program: Option<String>,
    pub account: Option<String>,
    pub hdkeypath: Option<String>,
    pub ismine: Option<bool>,
    pub iswatchonly: Option<bool>,
}