use std::fmt::Debug;
use rustomaton::StateEnum;
use druid::Color;
use crate::state::{StateEnum, EnumSize, State};



#[derive(Clone, Copy, Debug, druid::Data, Default, PartialEq, Eq,StateEnum)]
#[repr(usize)]
pub enum Gol {
    #[default]
    #[color("000000")]
    Dead,
    #[color("FFFFFF")]
    Alive,
}

impl State for Gol {
    fn transition(&self, neighbours: &[usize; Self::N_STATES]) -> Self {
        match (self, neighbours) {
            (Gol::Alive, [_, 2]) | (Gol::Alive, [_, 3]) | (Gol::Dead, [_, 3]) => Gol::Alive,
            _ => Gol::Dead,
        }
    }
}


