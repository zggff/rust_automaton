use std::fmt::Debug;
use druid::Color;

pub trait EnumSize {
    const N_STATES: usize;
}

pub trait StateEnum: Sized + Default + Debug + Copy + Clone + PartialEq + Eq + EnumSize
where [(); Self::N_STATES]: Sized
{
    const TITLES: [(&'static str, usize); Self::N_STATES];
    const COLOURS: [Color; Self::N_STATES];
    fn as_usize(&self) -> usize;
    fn from_usize(s: usize) -> Self;

}

pub trait State: StateEnum
where
    [(); <Self as EnumSize>::N_STATES]: Sized,
{
    fn transition(&self, neighbours: &[usize; Self::N_STATES]) -> Self;
}
