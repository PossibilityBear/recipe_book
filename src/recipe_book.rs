use std::fmt;
use crate::amount::{Amount};
use bincode;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub quantity: Amount,
}

impl fmt::Display for Ingredient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.name, self.quantity)
    }
}




