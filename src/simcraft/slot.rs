
use std::cmp::{PartialEq};
use std::default::Default;
use std::result::{Result};
use std::io::{Error, ErrorKind};


#[derive(PartialEq, PartialOrd, Eq, Hash, Clone, Copy, Debug)]
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
    Finger,
    Finger1,
    Finger2,
    Trinket,
    Trinket1,
    Trinket2,
    MainHand,
    OffHand
}


#[derive(PartialOrd, Hash, Clone, Debug)]
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
        let eslot: ESlot = match Slot::_get_eslot(name) {
            Ok(s) => Slot::fix_slot(&s),
            Err(err) => {
                return Err(Error::new(ErrorKind::InvalidInput, err));
            }
        };

        Ok(Slot {
            slot: eslot,
            name: Slot::_get_name(&eslot)
        })
    }

    pub fn fix_slot(eslot: &ESlot) -> ESlot {
        match eslot {
            ESlot::Finger1 => ESlot::Finger,
            ESlot::Finger2 => ESlot::Finger,
            ESlot::Trinket1 => ESlot::Trinket,
            ESlot::Trinket2 => ESlot::Trinket,
            _ => eslot.clone()
        }
    }

    pub fn get_real_slot(slot: &Slot, part: u8) -> Result<Slot, Error> {
        let eslot = match slot.slot {
            ESlot::Trinket => {
                match part {
                    1 => ESlot::Trinket1,
                    2 => ESlot::Trinket2,
                    _ => { return Err(Error::new(ErrorKind::InvalidInput, "Value of part is wrong")); }
                }
            },
            ESlot::Finger => {
                match part {
                    1 => ESlot::Finger1,
                    2 => ESlot::Finger2,
                    _ => { return Err(Error::new(ErrorKind::InvalidInput, "Value of part is wrong")); }
                }
            },
            _ => slot.slot.clone()
        };

        Ok(Slot {
            slot: eslot,
            name: Slot::_get_name(&eslot)
        })
    }

    fn _get_eslot(name: &str) -> Result<ESlot, Error> {
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
            "finger" => Ok(ESlot::Finger),
            "finger1" => Ok(ESlot::Finger1),
            "finger2" => Ok(ESlot::Finger2),
            "trinket" => Ok(ESlot::Trinket),
            "trinket1" => Ok(ESlot::Trinket1),
            "trinket2" => Ok(ESlot::Trinket2),
            "main_hand" => Ok(ESlot::MainHand),
            "off_hand" => Ok(ESlot::OffHand),
            _ => Err(Error::new(ErrorKind::InvalidInput, String::from("Invalid slot name")))
        }
    }

    fn _get_name(slot: &ESlot) -> String {
        match slot {
            ESlot::Head => String::from("head"),
            ESlot::Neck => String::from("neck"),
            ESlot::Shoulder => String::from("shoulder"),
            ESlot::Back => String::from("back"),
            ESlot::Chest => String::from("chest"),
            ESlot::Wrist => String::from("wrist"),
            ESlot::Waist => String::from("waist"),
            ESlot::Hands => String::from("hands"),
            ESlot::Legs => String::from("legs"),
            ESlot::Feet => String::from("feet"),
            ESlot::Finger => String::from("finger"),
            ESlot::Finger1 => String::from("finger1"),
            ESlot::Finger2 => String::from("finger2"),
            ESlot::Trinket => String::from("trinket"),
            ESlot::Trinket1 => String::from("trinket1"),
            ESlot::Trinket2 => String::from("trinket2"),
            ESlot::MainHand => String::from("main_hand"),
            ESlot::OffHand => String::from("off_hand"),
            _ => String::new()
        }
    }
}
