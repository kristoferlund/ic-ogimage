use crate::{animals::add_animals, certified_data::init_assets};

#[ic_cdk::init]
async fn init() {
    add_animals();
    init_assets();
}

#[ic_cdk::post_upgrade]
fn upgrade() {
    add_animals();
    init_assets();
}
