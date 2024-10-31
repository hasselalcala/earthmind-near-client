use std::time::Duration;

pub const ACCOUNT_TO_LISTEN: &str = "earthmindprotocol.testnet";
pub const FUNCTION_TO_LISTEN: &str = "request_governance_decision";
pub const DB_PATH: &str = "./data";
pub const LAST_PROCESSED_BLOCK_KEY: &[u8] = b"last_processed_block";
pub const NEAR_RPC_TESTNET: &str = "https://rpc.testnet.pagoda.co";
pub const NEAR_RPC_MAINNET: &str = "https://rpc.mainnet.pagoda.co";
pub const EARTHMIND_PROTOCOL_CONTRACT_TESTNET: &str = "earthmindprotocol.testnet";
pub const EARTHMIND_PROTOCOL_CONTRACT_MAINNET: &str = "earthmindprotocol.near";
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);
