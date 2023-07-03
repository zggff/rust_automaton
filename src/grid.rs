use crate::state::State;

use super::{HEIGHT, WIDTH};

#[derive(Clone, Debug, druid::Data)]
pub struct Grid<T: State>
where
    [(); T::N_STATES]: Sized,
{
    grid: [[T; WIDTH]; HEIGHT],
}

impl<T: State> Grid<T>
where
    [(); T::N_STATES]: Sized,
{
    pub fn new() -> Self {
        Grid {
            grid: [[T::default(); WIDTH]; HEIGHT],
        }
    }
    pub fn get(&self, x: usize, y: usize) -> T {
        self.grid[y][x]
    }
    pub fn set(&mut self, x: usize, y: usize, val: T) {
        self.grid[y][x] = val;
    }
    pub fn update(&mut self) {
        let mut grid = [[T::default(); WIDTH]; HEIGHT];
        #[allow(clippy::needless_range_loop)]
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                grid[y][x] = self.get(x, y).transition(&self.get_neighbours(x, y));
            }
        }
        self.grid = grid;
    }
    fn get_neighbours(&self, x: usize, y: usize) -> [usize; T::N_STATES] {
        let mut res = [0; T::N_STATES];
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let mut x0 = (x as isize) + i;
                let mut y0 = (y as isize) + j;
                if x0 < 0 {
                    x0 = WIDTH as isize - 1;
                }
                if y0 < 0 {
                    y0 = HEIGHT as isize - 1;
                }
                if x0 >= WIDTH as isize {
                    x0 = 0
                }
                if y0 >= HEIGHT as isize {
                    y0 = 0
                }
                res[self.get(x0 as usize, y0 as usize).as_usize()] += 1;
            }
        }
        res
    }
}

impl<T: State> Default for Grid<T>
where
    [(); T::N_STATES]: Sized,
{
    fn default() -> Self {
        Self::new()
    }
}
