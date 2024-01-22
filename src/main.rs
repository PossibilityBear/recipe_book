mod recipe_book;
mod amount;



use crate::{recipe_book::Ingredient, amount::{UnitType, Amount}};



fn main() {
    // let unqlite = UnQlite::create_temp();

    // UnQLite.kv_store("cheese", {data})
    let cheese = Ingredient {
        name: String::from("Cheese"),
        quantity: Amount{
           quantity: 15.0,
           unit: UnitType::Volume(amount::VolumeUnits::Cup), 
        }
    };

    println!("{}", cheese);
}
