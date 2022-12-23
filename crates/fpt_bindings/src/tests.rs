#[allow(unused_imports)]
use super::*;

#[test]
#[ignore = "avoid spam"]
fn test_sparebank1_get_access_token() {
    let mut config = fpt_common::Config::load().unwrap();
    sparebank1::get_access_token(&mut config).unwrap();
}

#[test]
#[ignore = "avoid spam"]
fn test_sparebank1() {
    let mut config = fpt_common::Config::load().unwrap();
    sparebank1::get_assets(&mut config).unwrap();
}

#[test]
#[ignore = "avoid spam"]
fn test_currency() {
    currency::get_value("btc", "usd").unwrap();
}

#[test]
#[ignore = "avoid spam"]
fn test_btccom() {
    btccom::get_balance("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa").unwrap();
}

#[test]
#[ignore = "avoid spam"]
fn test_ethplorer() {
    ethplorer::get_balance("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045").unwrap();
}

#[test]
#[ignore = "avoid spam"]
fn test_coinbase() {
    let config = fpt_common::Config::load().unwrap();
    coinbase::get_assets(&config).unwrap();
}
