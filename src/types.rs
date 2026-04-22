use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Chain {
    Bitcoin,
    Litecoin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Network {
    Mainnet,
    Testnet,
    Regtest,
    Signet,
}

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
    #[serde(rename = "redeemScript")]
    pub redeem_script: Option<String>,
    #[serde(rename = "witnessScript")]
    pub witness_script: Option<String>,
    pub spendable: bool,
    pub solvable: bool,
    pub reused: Option<bool>,
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
    pub fee: Option<f64>,
    pub confirmations: i64,
    pub generated: Option<bool>,
    pub trusted: Option<bool>,
    pub blockhash: Option<String>,
    pub blockheight: Option<u64>,
    pub blockindex: Option<u64>,
    pub blocktime: Option<u64>,
    pub time: u64,
    pub timereceived: u64,
    pub comment: Option<String>,
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: Option<String>,
    pub abandoned: Option<bool>,
    #[serde(rename = "involvesWatchonly")]
    pub involves_watchonly: Option<bool>,
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
    #[serde(rename = "txinwitness")]
    pub txin_witness: Option<Vec<String>>,
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

    pub in_active_chain: Option<bool>,
    pub blockhash: Option<String>,
    pub confirmations: Option<u64>,
    pub blocktime: Option<u64>,
    pub time: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockchainInfo {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub bestblockhash: String,
    pub difficulty: f64,
    pub mediantime: u64,
    pub verificationprogress: f64,
    pub initialblockdownload: bool,
    pub chainwork: String,
    pub size_on_disk: u64,
    pub pruned: bool,

    pub pruneheight: Option<u64>,
    pub automatic_pruning: Option<bool>,
    pub prune_target_size: Option<u64>,

    pub softforks: Option<serde_json::Value>,
    pub warnings: Option<String>,
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

    pub ismine: Option<bool>,
    pub iswatchonly: Option<bool>,
    pub solvable: Option<bool>,
    pub desc: Option<String>,
    pub pubkey: Option<String>,
    pub iscompressed: Option<bool>,
    pub labels: Option<Vec<String>>,

    pub hdkeypath: Option<String>,

    pub error: Option<String>,
    pub error_locations: Option<Vec<usize>>,
}

// implementations

impl TransactionListItem {
    pub fn is_send(&self) -> bool {
        self.category == "send"
    }
}