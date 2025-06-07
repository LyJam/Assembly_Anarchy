use crate::game::*;
use std::fmt;

#[derive(Component, Debug, Copy, Clone, PartialEq)]
pub enum Item {
    Bolt,
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
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            Item::Bolt => "Bolt",
        }
    }
}
