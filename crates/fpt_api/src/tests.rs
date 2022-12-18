#[allow(unused_imports)]
use super::*;

#[test]
fn test_add_asset_to_database() {
    let db = Database::open().unwrap();
    db.add_asset(&structures::Asset::default()).unwrap();
}

#[test]
#[ignore = "avoid spam"]
fn test_sparebank1_get_access_token() {
    let mut config = Config::load().unwrap();
    sparebank1_api::get_access_token(&mut config).unwrap();
}

#[test]
#[ignore = "avoid spam"]
fn test_sparebank1_get_assets() {
    let mut config = Config::load().unwrap();
    sparebank1_api::get_assets(&mut config).unwrap();
}

#[test]
#[ignore = "avoid spam"]
fn test_currency_get_value() {
    currency_api::get_value("btc", "usd").unwrap();
}

#[test]
#[ignore = "avoid spam"]
fn test_btccom_get_balance() {
    btccom_api::get_balance("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa").unwrap();
}

#[test]
#[ignore = "avoid spam"]
fn test_ethplorer_get_balance() {
    ethplorer_api::get_balance("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045").unwrap();
}
