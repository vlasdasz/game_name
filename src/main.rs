#![feature(default_free_fn)]

mod controls_view;
mod game_level;

use test_engine::{tools::Boxed, Screen};

use crate::controls_view::ControlsView;

fn main() {
    let mut screen = Screen::new((1000, 680).into());
    screen.ui.set_view(ControlsView::boxed());
    screen.ui.add_debug_view();
    screen.start_main_loop();
}
