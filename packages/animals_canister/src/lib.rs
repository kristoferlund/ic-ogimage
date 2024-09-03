mod animals;
mod certified_data;
mod declarations;
mod http;
mod service;

use animals::Animal;
use asset_util::CertifiedAssets;
use canister_sig_util::signature_map::SignatureMap;
use std::{cell::RefCell, collections::HashMap};

thread_local! {
    static ANIMALS: RefCell<HashMap<u32, Animal>> = RefCell::new(HashMap::new());
    static SIGNATURES : RefCell<SignatureMap> = RefCell::new(SignatureMap::default());
    static ASSETS: RefCell<CertifiedAssets> = RefCell::new(CertifiedAssets::default());
}
