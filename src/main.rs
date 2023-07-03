#![feature(if_let_guard)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

#[allow(unused)]
use automatons::{Bb, Gol, Seeds};
use druid::widget::{
    prelude::*, Checkbox, Flex, MainAxisAlignment, RadioGroup, SizedBox, Slider,
};
use druid::{AppLauncher, LocalizedString, WidgetExt, WindowDesc};
use grid_display::{AppState, GridDisplay};
use state::State;

const WIDGET_WIDTH: usize = 600;
const WIDTH: usize = 30;
const HEIGHT: usize = WIDTH;
const CELL_SIZE: f64 = WIDGET_WIDTH as f64 / WIDTH as f64;

mod automatons;
mod grid;
mod grid_display;
mod state;

pub fn main() {
    let window = WindowDesc::new(build_root_widget::<Bb>())
        .title(LocalizedString::new("automatons"))
        .window_size((800.0, 1000.0))
        .resizable(false);

    AppLauncher::with_window(window)
        .launch(AppState::new())
        .expect("launch failed");
}

fn build_root_widget<T: State + druid::Data>() -> impl Widget<AppState<T>>
where
    [(); T::N_STATES]: Sized,
{
    let radio = RadioGroup::column(T::TITLES).lens(AppState::target);
    let toggle = Checkbox::new("Run").lens(AppState::run);
    let delta = Slider::new()
        .with_range(1.0, 8.0)
        .with_step(0.5)
        .lens(AppState::fps);
    let grid = SizedBox::new(GridDisplay::new())
        .width(WIDTH as f64 * CELL_SIZE)
        .height(HEIGHT as f64 * CELL_SIZE);
    let brush_controls = radio.padding(10.0);
    let play_controls = Flex::column()
        .with_child(toggle)
        .with_spacer(10.0)
        .with_child(delta)
        .padding(10.0);
    let controls = Flex::row()
        .main_axis_alignment(MainAxisAlignment::SpaceEvenly)
        .with_child(brush_controls)
        .with_child(play_controls)
        .fix_width(WIDTH as f64 * CELL_SIZE);

    Flex::column()
        .with_child(controls)
        .with_spacer(20.0)
        .with_child(grid)
        .center()
}
