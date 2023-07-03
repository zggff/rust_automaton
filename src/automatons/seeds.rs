use std::fmt::Debug;
use automaton::StateEnum;
use druid::Color;
use crate::state::{StateEnum, EnumSize, State};


#[derive(Clone, Copy, Debug, druid::Data, Default, PartialEq, Eq, StateEnum)]
#[repr(usize)]
pub enum Seeds {
    #[default]
    #[color("000000")]
    Dead,
    #[color("FFFFFF")]
    Alive,
}

impl State for Seeds {
    fn transition(&self, neighbours: &[usize; Self::N_STATES]) -> Self {
        match (self, neighbours) {
            (Self::Dead, [_, 2]) => Self::Alive,
            _ => Self::Dead,
        }
    }
}
