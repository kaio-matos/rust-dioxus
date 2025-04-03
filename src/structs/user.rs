use serde::{Deserialize, Serialize};

#[derive(Debug,Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub email: String,
    pub address: Address,
    pub phone: String,
    pub website: String,
    pub company: Company,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub suite: String,
    pub city: String,
    pub zipcode: String,
    pub geo: Geo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Geo {
    pub lat: String, // Using String since the JSON has it as a string (e.g., "-71.4197")
    pub lng: String, // Same reasoning
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Company {
    pub name: String,
    #[serde(rename = "catchPhrase")]
    pub catch_phrase: String, // Rename to follow Rust naming conventions
    pub bs: String,
}
