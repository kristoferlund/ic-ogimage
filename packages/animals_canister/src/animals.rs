use candid::CandidType;
use serde::Deserialize;

use crate::ANIMALS;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct Animal {
    pub id: u32,
    pub name: String,
    pub emoji: String,
}

pub fn add_animals() {
    ANIMALS.with(|things| {
        things.borrow_mut().insert(
            1,
            Animal {
                id: 1,
                name: "Cat".to_string(),
                emoji: "🐈".to_string(),
            },
        );
        things.borrow_mut().insert(
            2,
            Animal {
                id: 2,
                name: "Dog".to_string(),
                emoji: "🐕".to_string(),
            },
        );
        things.borrow_mut().insert(
            3,
            Animal {
                id: 3,
                name: "Horse".to_string(),
                emoji: "🐎".to_string(),
            },
        );
        things.borrow_mut().insert(
            4,
            Animal {
                id: 4,
                name: "Lizard".to_string(),
                emoji: "🦎".to_string(),
            },
        );
    });
}
