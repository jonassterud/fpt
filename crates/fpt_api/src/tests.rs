#[allow(unused_imports)]
use super::*;

#[test]
fn test_add_asset_to_database() {
    let db = Database::open().unwrap();
    db.add_asset(&structures::Asset::default()).unwrap();
}

#[test]
fn test_sparebank1_get_access_token() {
    sparebank1_api::get_access_token().unwrap();
}

#[test]
fn test_sparebank1_get_assets() {
    sparebank1_api::get_assets().unwrap();
}
