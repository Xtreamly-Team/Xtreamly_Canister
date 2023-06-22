pub const MAX_RESPONSE_BYTES: u64 = 10 * 6 * 200;
pub const URL: &str = "https://chainnova-scf-chain.darkube.app";
pub const CHAIN_ID: u64 = 1337;
pub const KEY_NAME: &str = "dfx_test_key";
pub const ERC20_TOKEN_ABI: &[u8] = include_bytes!("../contracts/erc20.abi.json");
pub const VC_ABI: &[u8] = include_bytes!("../contracts/vc.abi");
