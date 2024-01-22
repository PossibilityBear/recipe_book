use std::fmt::{Display, Formatter, Result};

pub trait Quantity {
    fn count(&self) -> f64;
    fn unit(&self) -> UnitType;
}

impl Display for dyn Quantity {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}, {}", self.count(), self.unit())
    }
}

#[derive(Debug, Clone)]
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
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.unit_type())
    }
}


#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

pub struct Amount {
    pub quantity: f64,
    pub unit: UnitType, 
}

impl Display for Amount {
    fn fmt(&self, f: &mut Formatter) -> Result {
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


