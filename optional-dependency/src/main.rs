#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Role {
    Admin,
    Standard,
    #[default]
    Guest,
}

#[derive(Debug, Clone, Default, PartialEq)]
struct DB {}

#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct User {
    id: u32,
    name: String,
    role: Role,
    #[cfg_attr(feature = "serde", serde(skip))]
    db: Arc<DB>,
}

fn main() {
    let user: User = Default::default();
    println!("Default user: {:?}", user);
    #[cfg(feature = "serde")]
    {
        let user_str = serde_json::to_string(&user).unwrap();
        let user_de: User = serde_json::from_str(&user_str).unwrap();
        println!("Deserialized user: {:?}", user_de);

        assert_eq!(user, user_de);
    }
}
