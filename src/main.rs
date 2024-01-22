mod recipe_book;
mod amount;

use std::{result, io::Bytes};

use bincode::deserialize_from_custom;
use serde::Serialize;
use unqlite::{UnQLite, KV, Cursor};

use crate::{recipe_book::Ingredient, amount::{UnitType, Amount}};



fn main() {

    let cheese = Ingredient {
        name: String::from("Cheese"),
        quantity: Amount{
           quantity: 15.0,
           unit: UnitType::Volume(amount::VolumeUnits::Cup), 
        }
    };

    println!("{}", cheese);

    // test serialization and desirialization
    let encoded: Vec<u8> = bincode::serialize(&cheese).unwrap();
    
    let decoded: Ingredient = bincode::deserialize(&encoded[..]).unwrap();

    println!("{}", decoded);
    
    // test basic temporary db

    let unqlite = UnQLite::create_temp();
    // Use any type that can use as `[u8]`
    unqlite.kv_store("cheese", bincode::serialize(&cheese).unwrap());

    let mut entry = unqlite.first();
    // Iterate records
    loop {
        if entry.is_none() { break; }

        let record = entry.expect("valid entry");
        let (key, value) = record.key_value();
        let value: Ingredient = bincode::deserialize(&value[..]).unwrap();
        println!("Value: {}", value);

        let key: String = String::from_utf8(key).unwrap();

        println!("Key: {:?}", key);

        entry = record.next();
    }

}
