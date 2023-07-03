use std::fmt::Debug;
use automaton::StateEnum;
use druid::Color;
use crate::state::{StateEnum, EnumSize, State};

#[derive(Clone, Copy, Debug, druid::Data, Default, PartialEq, Eq, StateEnum)]
#[repr(usize)]
pub enum Bb {
    #[default]
    #[color("000000")]
    Dead,
    #[color("FFFFFF")]
    Alive,
    #[color("00AA00")]
    Dying,
}

impl State for Bb {
    fn transition(&self, neighbours: &[usize; Self::N_STATES]) -> Self {
        match (self, neighbours) {
            (Self::Dead, [_, 2, _]) => Self::Alive,
            (Self::Alive, _) => Self::Dying,
            (Self::Dying, _) => Self::Dead,
            _ => Self::Dead,
        }
    }
    
}
