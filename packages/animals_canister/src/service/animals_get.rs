use ic_cdk::trap;

use crate::{animals::Animal, ANIMALS};

#[ic_cdk::query]
pub fn animals_get(id: u32) -> Animal {
    match ANIMALS.with_borrow(|things| things.get(&id).cloned()) {
        Some(thing) => thing,
        None => trap("Animal not found"),
    }
}
