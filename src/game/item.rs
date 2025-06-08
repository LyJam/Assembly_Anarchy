use crate::game::*;
use std::fmt;

#[derive(Component, Debug, Copy, Clone, PartialEq)]
pub enum Item {
    Bolt,
    Steel,
    Gold,
    Jewelry,
    Iron,
    Wrench,
    Crystal,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_name())
    }
}

impl Item {
    pub fn get_sprite(&self) -> &'static str {
        match self {
            Item::Bolt => "Items/bolt.png",
            Item::Steel => "Items/steel.png",
            Item::Gold => "Items/gold.png",
            Item::Jewelry => "Items/jewelry.png",
            Item::Iron => "Items/iron.png",
            Item::Wrench => "Items/wrench.png",
            Item::Crystal => "Items/crystal.png",
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            Item::Bolt => "bolts",
            Item::Steel => "steel",
            Item::Gold => "gold",
            Item::Jewelry => "jewelry",
            Item::Iron => "iron",
            Item::Wrench => "wrench",
            Item::Crystal => "crystal",
        }
    }
}
