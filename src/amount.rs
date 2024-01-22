use std::fmt::{Display, Formatter};
use std::fmt;
use bincode;
use serde::{Serialize, Deserialize};
use serde::ser::{SerializeStruct, Serializer};

pub trait Quantity {
    fn count(&self) -> f64;
    fn unit(&self) -> UnitType;
}

impl Display for dyn Quantity {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.count(), self.unit())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnitType{
    Mass(MassUnits),
    Volume(VolumeUnits),
}

pub trait Unit {
    fn unit_type(&self) -> UnitType;
}

impl Unit for UnitType{
    fn unit_type(&self) -> UnitType {
        self.clone()
    }
}

impl Display for UnitType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.unit_type())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VolumeUnits {
    Gallon,
    Quart,
    Pint,
    Cup,
    Tbsp,
    Tsp,
    FlOz,
    Ml,
    L,
}

impl Unit for VolumeUnits {
    fn unit_type(&self) -> UnitType {
        UnitType::Volume(self.clone())
    } 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MassUnits {
    Oz,
    Lb,
    G,
    Kg,
}

impl Unit for MassUnits {
    fn unit_type(&self) -> UnitType {
        UnitType::Mass(self.clone())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Amount {
    pub quantity: f64,
    pub unit: UnitType, 
}

impl Display for Amount {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.quantity, self.unit)
    }
}

impl Quantity for Amount {
    fn count(&self) -> f64 {
        self.quantity
    }

    fn unit(&self) -> UnitType {
        self.unit.unit_type()
    }

    
    // fn unit(&self) -> &str {
    //     format!("{:?}", self.unit).to_str()
    // }
}

// impl Serialize for Amount {
//     fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
//         let mut s = serializer.serialize_struct("Amount", 2)?;
//         s.serialize_field("quantity", &self.quantity)?;
//         s.serialize_field("unit", &self.unit)?;
//         s.end()
//     }
// }