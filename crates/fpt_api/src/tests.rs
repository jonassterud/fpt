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
