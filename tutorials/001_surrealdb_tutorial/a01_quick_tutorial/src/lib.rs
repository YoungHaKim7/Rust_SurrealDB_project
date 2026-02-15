use fake::faker::name::raw::*;
use fake::{Fake, locales::*};
use rand::random_range;
use std::num::ParseIntError;
use surrealdb_types::SurrealValue;

#[derive(Debug, SurrealValue)]
pub struct User {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: i32,
}

fn can_drive(age: i64) -> bool {
    age >= 18
}

fn parse_number(input: String) -> Result<i64, ParseIntError> {
    input.parse::<i64>()
}

pub fn random_user() -> User {
    User {
        first_name: FirstName(EN).fake(),
        middle_name: FirstName(FR_FR).fake(),
        last_name: LastName(DE_DE).fake(),
        age: random_range(10..=50),
    }
}
