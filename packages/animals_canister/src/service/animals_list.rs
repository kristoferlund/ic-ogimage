use crate::{animals::Animal, ANIMALS};

#[ic_cdk::query]
pub fn animals_list() -> Vec<Animal> {
    ANIMALS.with_borrow(|things| things.values().cloned().collect())
}
