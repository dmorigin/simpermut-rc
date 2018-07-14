
use std::cmp::{PartialEq, PartialOrd};
use std::default::Default;
use std::result::{Result};
use std::io::{Error, ErrorKind};


#[derive(PartialEq, PartialOrd, Eq, Hash, Clone)]
pub enum ESlot
{
    Unkown,
    Head,
    Neck,
    Shoulder,
    Back,
    Chest,
    Wrist,
    Hands,
    Waist,
    Legs,
    Feet,
    Finger1,
    Finger2,
    Trinket1,
    Trinket2,
    MainHand,
    OffHand
}

#[derive(PartialOrd, Hash, Clone)]
pub struct Slot
{
    pub slot: ESlot,
    pub name: String
}


impl Default for Slot {
    fn default() -> Slot {
        Slot {
            slot: ESlot::Unkown,
            name: String::new()
        }
    }
}

impl PartialEq for Slot {
    fn eq(&self, other: &Slot) -> bool {
        self.slot == other.slot
    }
}

impl Eq for Slot {
}



impl Slot {
    pub fn new() -> Slot {
        Slot {
            slot: ESlot::Unkown,
            name: String::new()
        }
    }

    pub fn from_str(name: &str) -> Result<Slot, Error> {
        let eslot: ESlot = match Slot::_get(name) {
            Ok(s) => s,
            Err(err) => {
                return Err(Error::new(ErrorKind::InvalidInput, err));
            }
        };

        Ok(Slot {
            slot: eslot,
            name: String::from(name)
        })
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn slot(&self) -> ESlot {
        self.slot.clone()
    }

    fn _get(name: &str) -> Result<ESlot, Error> {
        match name {
            "head" => Ok(ESlot::Head),
            "neck" => Ok(ESlot::Neck),
            "shoulder" => Ok(ESlot::Shoulder),
            "back" => Ok(ESlot::Back),
            "chest" => Ok(ESlot::Chest),
            "wrist" => Ok(ESlot::Wrist),
            "waist" => Ok(ESlot::Waist),
            "hands" => Ok(ESlot::Hands),
            "feet" => Ok(ESlot::Feet),
            "legs" => Ok(ESlot::Legs),
            "finger1" => Ok(ESlot::Finger1),
            "finger2" => Ok(ESlot::Finger2),
            "trinket1" => Ok(ESlot::Trinket1),
            "trinket2" => Ok(ESlot::Trinket2),
            "main_hand" => Ok(ESlot::MainHand),
            "off_hand" => Ok(ESlot::OffHand),
            _ => Err(Error::new(ErrorKind::InvalidInput, String::from("Invalid slot name")))
        }
    }
}
