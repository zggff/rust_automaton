use std::process::exit;
use std::time::Duration;

use druid::keyboard_types::Key;
use druid::widget::prelude::*;
use druid::{KeyEvent, Lens, MouseEvent, Rect, TimerToken};

use crate::grid::Grid;
use crate::state::State;

use crate::{CELL_SIZE, HEIGHT, WIDTH};

#[derive(Debug, Clone, Default, druid::Data, Lens)]
pub struct AppState<T: State>
where
    [(); T::N_STATES]: Sized,
{
    target: usize,
    run: bool,
    fps: f64,

    grid: Grid<T>,
}

impl<T: State> AppState<T>
where
    [(); T::N_STATES]: Sized,
{
    pub fn new() -> Self {
        AppState {
            fps: 5.0,
            ..AppState::default()
        }
    }
}

pub struct GridDisplay {
    timer_id: TimerToken,
}

impl GridDisplay {
    pub fn new() -> GridDisplay {
        GridDisplay {
            timer_id: TimerToken::INVALID,
        }
    }
}

impl<T: State> Widget<AppState<T>> for GridDisplay
where
    [(); T::N_STATES]: Sized,
{
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState<T>, _env: &Env) {
        match event {
            Event::WindowConnected => ctx.set_focus(ctx.widget_id()),
            Event::MouseMove(MouseEvent { pos,buttons, ..}) => {
                if buttons.has_left() {
                    let x = (pos.x / CELL_SIZE).floor() as usize;
                    let y = (pos.y / CELL_SIZE).floor() as usize;
                    data.grid.set(x, y, T::from_usize(data.target));
                    ctx.request_paint();
                }
            }
            Event::MouseDown(MouseEvent { pos, .. }) => {
                let x = (pos.x / CELL_SIZE).floor() as usize;
                let y = (pos.y / CELL_SIZE).floor() as usize;
                    data.grid.set(x, y, T::from_usize(data.target));
                ctx.request_paint();

            }
            Event::Timer(id) => {
                if *id == self.timer_id && data.run {
                    data.grid.update();
                    ctx.request_paint();
                    self.timer_id = ctx.request_timer(Duration::from_secs_f64(1.0 / 2.0_f64.powf(data.fps)));

                }
            }
            Event::KeyDown(KeyEvent { key, .. }) if let Key::Escape = key => exit(0),
            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        _data: &AppState<T>,
        _env: &Env,
    ) {
        if let LifeCycle::BuildFocusChain = event {
            ctx.register_for_focus()
        }
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: &AppState<T>,
        data: &AppState<T>,
        _env: &Env,
    ) {
        if data.run && !old_data.run {
            self.timer_id =
                ctx.request_timer(Duration::from_secs_f64(1.0 / 2.0_f64.powf(data.fps)));
        }
        ctx.request_paint()
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppState<T>,
        _env: &Env,
    ) -> Size {
        if bc.is_width_bounded() && bc.is_height_bounded() {
            bc.max()
        } else {
            let size = Size::new(100.0, 100.0);
            bc.constrain(size)
        }
    }
    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState<T>, _env: &Env) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let x0 = x as f64 * CELL_SIZE;
                let y0 = y as f64 * CELL_SIZE;
                let rect = Rect::new(x0, y0, x0 + CELL_SIZE, y0 + CELL_SIZE);
                let fg = T::COLOURS[data.grid.get(x, y).as_usize()];
                ctx.fill(rect, &fg)
            }
        }
    }
}
